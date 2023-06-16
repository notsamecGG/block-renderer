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



pub struct CameraController {
    pressed_keys: [bool; 6],
    translation: glam::Vec3,
    speed: f32,
    forward: glam::Vec3,
    right: glam::Vec3,
    up: glam::Vec3,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(translation: glam::Vec3, sensitivity: f32, speed: f32) -> Self {
        Self {
            pressed_keys: [false; 6],
            translation,
            speed,
            forward: glam::Vec3::NEG_Z,
            right: glam::Vec3::X,
            up: glam::Vec3::Y,
            sensitivity,
        }
    }

    pub fn handle_keyboard_input(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::KeyboardInput { 
                input,
                ..
            } => {
                let is_pressed = input.state == winit::event::ElementState::Pressed;

                match input.virtual_keycode {
                    Some(winit::event::VirtualKeyCode::W) => {
                        self.pressed_keys[0] = is_pressed;
                        true
                    },
                    Some(winit::event::VirtualKeyCode::S) => {
                        self.pressed_keys[1] = is_pressed;
                        true
                    },
                    Some(winit::event::VirtualKeyCode::A) => {
                        self.pressed_keys[2] = is_pressed;
                        true
                    },
                    Some(winit::event::VirtualKeyCode::D) => {
                        self.pressed_keys[3] = is_pressed;
                        true
                    },
                    Some(winit::event::VirtualKeyCode::Space) => {
                        self.pressed_keys[4] = is_pressed;
                        true
                    },
                    Some(winit::event::VirtualKeyCode::LControl) => {
                        self.pressed_keys[5] = is_pressed;
                        true
                    },
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn handle_pressed_keys(&mut self, delta: f32) {
        let mut local_speed = glam::Vec3::ZERO;

        if self.pressed_keys[0] {
            local_speed += self.speed * self.forward;
        }
        if self.pressed_keys[1] {
            local_speed += -self.speed * self.forward;
        }

        if self.pressed_keys[2] {
            local_speed += -self.speed * self.right;
        }
        if self.pressed_keys[3] {
            local_speed += self.speed * self.right;
        }

        if self.pressed_keys[4] {
            local_speed += self.speed * self.up;
        }
        if self.pressed_keys[5] {
            local_speed += -self.speed * self.up;
        }
        
        // let delta_translation = self.rotation * local_speed * delta;
        let delta_translation = local_speed * delta;
        self.translation += delta_translation;
    }

    pub fn handle_mouse_input(&mut self, input: &winit::event::DeviceEvent) -> bool {
        match input {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                let pitch_delta = delta.1 as f32 * self.sensitivity;
                let yaw_delta = delta.0 as f32 * self.sensitivity;

                let rotation = (glam::Quat::from_axis_angle(self.right, -pitch_delta.to_radians())  *
                    glam::Quat::from_axis_angle(glam::Vec3::Y, -yaw_delta.to_radians())).normalize();
                self.forward = rotation * self.forward;
                self.right = self.forward.cross(self.up).normalize();
                true
            },
            _ => false,
        }
    }
}



pub struct Camera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,

    controller: CameraController,

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

    pub fn translation(&self) -> glam::Vec3 {
        self.controller.translation
    }

    pub fn translation_mut(&mut self) -> &mut glam::Vec3 {
        &mut self.controller.translation
    }
}

impl Camera {
    fn calculate_aspect(state: &HardwareState) -> f32 {
        let window_size = state.window().inner_size();
        window_size.width as f32 / window_size.height as f32
    }

    pub fn new(
        state: &HardwareState,
        translation: glam::Vec3,
        fov: f32, 
        near: f32, 
        far: f32,
        sensitivity: f32,
        speed: f32,
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

        let camera_controller = CameraController::new(translation, sensitivity, speed);

        Camera {
            fov,
            aspect: Self::calculate_aspect(state),
            near,
            far,
            controller: camera_controller,
            uniform: camera_uniform,
            buffer: camera_buffer,
            bind_group_layout,
        }
    }

    fn build_view_projection(&self) -> glam::Mat4 {
        let view = glam::Mat4::look_at_rh(self.controller.translation, self.controller.translation + self.controller.forward, self.controller.up);
        let projection = glam::Mat4::perspective_rh_gl(self.fov, self.aspect, self.near, self.far);

        projection * view
    }

    pub fn update(&mut self, state: &HardwareState, delta_time: f32) {
        self.uniform.view_projection = self.build_view_projection().to_cols_array_2d();
        state.queue().write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
        self.controller.handle_pressed_keys(delta_time);
    }

    pub fn resize(&mut self, state: &HardwareState) {
        self.aspect = Self::calculate_aspect(state);
        self.update(state, 0.0);
    }

    pub fn handle_keyboard_input(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.controller.handle_keyboard_input(event)
    }

    pub fn handle_mouse_input(&mut self, input: &winit::event::DeviceEvent) -> bool {
        self.controller.handle_mouse_input(input)
    }
}
