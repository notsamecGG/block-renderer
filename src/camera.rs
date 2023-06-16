use wgpu::util::DeviceExt;

use crate::HardwareState;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { 
            view_projection: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection().to_cols_array_2d();
    }
}


pub struct Camera {
    position: glam::Vec3,
    rotation: glam::Quat,
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    uniform: CameraUniform,
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl Camera {
    pub fn create_bind_group(&self, state: &HardwareState) -> wgpu::BindGroup {
        state.device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.buffer.as_entire_binding(),
                    }
                ],
            }
        )
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}

impl Camera {
    fn calculate_aspect(state: &HardwareState) -> f32 {
        let window_size = state.window().inner_size();
        window_size.width as f32 / window_size.height as f32
    }

    pub fn new(
        state: &HardwareState,
        position: glam::Vec3, 
        rotation: glam::Quat, 
        fov: f32, 
        near: f32, 
        far: f32
    ) -> Self {
        let camera_uniform = CameraUniform::new();
        
        let camera_buffer = state.device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = state.device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        },
                        count: None,
                    }
                ],
            }
        );

        Camera {
            position,
            rotation,
            fov,
            aspect: Self::calculate_aspect(state),
            near,
            far,
            uniform: camera_uniform,
            buffer: camera_buffer,
            bind_group_layout,
        }
    }

    fn build_view_projection(&self) -> glam::Mat4 {
        let view = glam::Mat4::from_rotation_translation(self.rotation, self.position);
        let projection = glam::Mat4::perspective_rh_gl(self.fov, self.aspect, self.near, self.far);

        projection * view
    }

    pub fn update(&mut self, state: &HardwareState) {
        self.uniform.view_projection = self.build_view_projection().to_cols_array_2d();
        state.queue().write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    pub fn resize(&mut self, state: &HardwareState) {
        self.aspect = Self::calculate_aspect(state);
        self.update(state);
    }
}
