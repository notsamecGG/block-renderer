use bitvec::prelude::*;
use wgpu::util::DeviceExt;

use crate::{Array3D, HardwareState, Descriptable};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadInstance { 
    pub data: u32, // x, y, z (in the chunk), and face index (0..6)
}

impl Descriptable for QuadInstance {
    const STEP_MODE: wgpu::VertexStepMode = wgpu::VertexStepMode::Instance;
    const SIZE: wgpu::BufferAddress = std::mem::size_of::<Self>() as wgpu::BufferAddress;

    fn attribs() -> &'static [wgpu::VertexAttribute] {
        &wgpu::vertex_attr_array![
            1 => Uint32,
        ]
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawChunk {
    pub position: [i32; 3],
}

/// A chunk is a 16x16x16 area of blocks.
pub struct Chunk {
    // blocks: Vec<Block>,
    pub block_data: Array3D, // 1 block = 1 bit
    faces: Vec<BitVec>, //  1 block = 6 faces = 6 bits 
    face_count: u32,

    instances: Vec<QuadInstance>,
    instance_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    pub position: glam::IVec3,
    _metadata: RawChunk,
    _metadata_buffer: wgpu::Buffer,
}

impl Chunk {
    pub fn face_count(&self) -> u32 {
        self.face_count
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(state: &HardwareState) -> wgpu::BindGroupLayout {
        let bind_group_layout = state.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { 
            label: Some("Chunk Metadata Bind Group Layout"),
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

        bind_group_layout
    }
}

impl Chunk {
    pub fn new(state: &HardwareState, origin: glam::IVec3, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let block_data = Array3D::new(16);
        let faces = vec![BitVec::new(); 6];

        let instances = Vec::new();
        let instance_buffer = state.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Chunk Instance Buffer"),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let metadata = RawChunk {
            position: origin.to_array(),
        };
        let metadata_buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Metadata Buffer"),
            contents: bytemuck::cast_slice(&[metadata]),
            usage: wgpu::BufferUsages::STORAGE,
        });
        
        let bind_group = state.device().create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Chunk Metadata Bind Group"),
            layout: bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: metadata_buffer.as_entire_binding(),
            }], 
        });


        Self {
            block_data,
            faces,
            face_count: 0,
            instances,
            instance_buffer,
            bind_group,
            position: origin,
            _metadata: metadata,
            _metadata_buffer: metadata_buffer
        }
    }

    fn generate_instances(&self, faces: &[BitVec; 6]) -> Vec<QuadInstance> {
        let faces_count = faces.iter().map(|face| face.count_ones()).sum::<usize>();
        let mut instances = Vec::with_capacity(faces_count);

        for i in 0..4096 {
            for j in 0..6 {
                if !faces[j][i] {
                    continue;
                }

                let x = i % 16;
                let y = (i / 16) % 16;
                let z = i / (16 * 16);

                let face_index = j;

                let positional_data = x | (y << 8) | (z << 16) | (face_index << 24);
                let positional_data = positional_data as u32;

                instances.push(QuadInstance {
                    data: positional_data,
                });
            }
        }

        instances
    }

    pub fn update_faces(&mut self, state: &HardwareState, neighbors: Vec<Option<BitVec>>) {
        let faces = self.block_data.get_faces(neighbors);
       
        if self.faces[..] == faces[..] {
            return;
        }

        let instances = self.generate_instances(&faces);
        let instance_buffer = state.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        self.instances = instances;
        self.instance_buffer = instance_buffer;
        self.face_count = self.instances.len() as u32;
    }
}

