use crate::Descriptable;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance {
    color: u8,
}

impl Descriptable for QuadInstance {
    const ATTRIBS: [wgpu::VertexAttribute] = wgpu::vertex_attr_array![
        0 => wgpu::VertexFormat::Uint8,
    ];

    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
}
