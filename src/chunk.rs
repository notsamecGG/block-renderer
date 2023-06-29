use bitvec::prelude::*;
use wgpu::util::DeviceExt;

use crate::{Array3D, HardwareState, Descriptable};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance { }

impl Descriptable for QuadInstance {
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
    const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    fn attribs() -> &'static [wgpu::VertexAttribute] {
        &wgpu::vertex_attr_array![]
    }
}


/// A chunk is a 16x16x16 area of blocks.
pub struct Chunk {
    // blocks: Vec<Block>,
    pub block_data: Array3D, // 1 block = 1 bit
    faces: Vec<BitVec>, //  1 block = 6 faces = 6 bits 
    merged_faces: BitVec, // 1block = 6 faces = 6 bits

    faces_buffer: wgpu::Buffer,
    face_count: u32,

    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl Chunk {
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn face_count(&self) -> u32 {
        self.face_count
    }
}

impl Chunk {
    pub fn new(state: &HardwareState, _origin: glam::Vec3) -> Self {
        let block_data = Array3D::new(16);
        let faces = vec![BitVec::new(); 6];
        let mut merged_faces = BitVec::new();
        merged_faces.reserve(6 * 16 * 16 * 16);

        let faces_buffer = state.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Chunk Faces Buffer"),
            size: 6 * 16 * 16 * 16,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = state.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Chunk Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = state.device().create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Chunk Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: faces_buffer.as_entire_binding(),
            }], 
        });

        Self {
            block_data,
            faces,
            face_count: 0,
            merged_faces,
            faces_buffer,
            bind_group,
            bind_group_layout
        }
    }

    fn generate_faces(&mut self, state: &HardwareState, faces: [BitVec; 6]) {
        let merged_faces = self.merge_faces(&faces);

        // let current_face_count = self.face_count;
        let new_face_count = merged_faces.count_ones() as u32;

        state.queue().write_buffer(
            &self.faces_buffer, 
            0, 
            bytemuck::cast_slice(merged_faces.as_raw_slice())
        );

        self.faces = faces.to_vec();
        self.merged_faces = merged_faces;

        self.face_count = new_face_count;
    }

    fn merge_faces(&self, faces: &[BitVec; 6]) -> BitVec {
        let mut merged_faces = BitVec::with_capacity(6 * 16 * 16 * 16);

        for i in 0..4096 {
            for j in 0..6 {
                merged_faces.push(faces[j][i]);
            }
        }

        merged_faces
    }

    pub fn update_faces(&mut self, state: &HardwareState) {
        let faces = self.block_data.get_faces();
       
        if self.faces[..] == faces[..] {
            return;
        }

        self.generate_faces(state, faces);
    }
}


// convert the data field to raw bytes without converting throughout bytemuck
pub struct SimpleChunk {
    data: BitVec,
    buffer: wgpu::Buffer,
}

impl SimpleChunk {
    pub fn new(state: &HardwareState) -> Self {
        let data = BitVec::new();
        let buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simple Chunk Buffer"),
            contents: bytemuck::cast_slice(data.as_raw_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        Self { 
            data, 
            buffer 
        }
    }

    pub fn bind_group(&self, state: &HardwareState) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
        let layout = state.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Simple Chunk Bind Group Layout"), 
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    count: None,
                    ty: wgpu::BindingType::Buffer { 
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, 
                        has_dynamic_offset: false, 
                        min_binding_size: None
                    }
                }
            ]
        });


        let bind_group = state.device().create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Simple Chunk Bind Group"),
            layout: &layout, 
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.buffer.as_entire_binding(),
            }]
        });

        (bind_group, layout)
    }

    pub fn write_buffer(&mut self, state: &HardwareState, data: BitVec) {
        self.buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simple Chunk Buffer"),
            contents: bytemuck::cast_slice(data.as_raw_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        self.data = data;
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}
