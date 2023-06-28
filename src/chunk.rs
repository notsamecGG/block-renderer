use bitvec::prelude::*;
use wgpu::util::DeviceExt;

use crate::{Array3D, HardwareState};

/// A chunk is a 16x16x16 area of blocks.
pub struct Chunk {
    // blocks: Vec<Block>,
    block_data: Array3D, // 1 block = 1 bit
    faces: Vec<BitVec>, // 6 faces = 6 bitslices
    face_count: u32,
}

impl Chunk {
    pub fn new() -> Self {
        let block_data = Array3D::new(16);
        let faces = vec![BitVec::new(); 6];

        Self {
            block_data,
            faces,
            face_count: 0,
        }
    }

    pub fn generate_faces(&mut self, _state: &HardwareState, faces: &[BitVec; 6]) {
        let current_face_count = self.face_count;
        let mut new_face_count = 0;

        for side in faces {
            new_face_count += side.count_ones();
        }

        let new_face_count = new_face_count as u32;
        if new_face_count != current_face_count {
            // todo
        }

        self.face_count = new_face_count;
        self.faces = faces.to_vec();
    }

    pub fn update_faces(&mut self, state: &HardwareState) -> [BitVec; 6] {
        let faces = self.block_data.get_faces();
       
        if self.faces[..] == faces[..] {
            return faces;
        }

        self.generate_faces(state, &faces);

        faces
    }
}

// #[repr(C)]
// #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct RawSimpleChunk {
//     data: [u32],
// }

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
