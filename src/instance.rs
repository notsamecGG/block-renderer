use crate::Descriptable;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance {
    id: u32
}

impl Descriptable for QuadInstance {
    fn attribs() -> &'static [wgpu::VertexAttribute] {
        wgpu::vertex_attr_array![
        ].as_slice()
    }

    const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
}
