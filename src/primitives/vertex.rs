use crate::Descriptable;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { position: [x, y, z] }
    }
}

impl Descriptable for Vertex {
    const ATTRIBS: [wgpu::VertexAttribute] = wgpu::vertex_attr_array![
        0 => Float32x3,
    ];

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Vertex;
}
