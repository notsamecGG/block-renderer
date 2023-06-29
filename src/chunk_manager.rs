use std::collections::VecDeque;
use bitvec::vec::BitVec;
use futures::future::join_all;

use crate::{Chunk, HardwareState};


fn check_is_chunk_visible(chunk: &Chunk, camera: &crate::Camera) -> bool {
    true
}

pub struct ChunkManager {
    position: glam::IVec3,
    view_distance: i32,

    chunk_bind_group_layout: wgpu::BindGroupLayout,

    active_chunks: VecDeque<Chunk>,
    to_load: VecDeque<Chunk>,
    to_unload: VecDeque<Chunk>,
}

impl ChunkManager {
    pub fn chunk_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.chunk_bind_group_layout
    }
}

impl ChunkManager {
    fn get_neighbors(&self) -> [&BitVec; 6] {
    }

    fn initialize_chunk(state: &HardwareState, chunk: &mut Chunk) {
        chunk.block_data.set(8, 8, 8, true);

        // for z in 0..16 {
        //     for y in 0..16 {
        //         for x in 0..16 {
        //             chunk.block_data.set(x, y, z, true);
        //         }
        //     }
        // }

        chunk.update_faces(&state);
    }

    pub fn initialize_chunks(&mut self, state: &HardwareState) {
        if self.to_load.is_empty() {
            return;
        }

        for _ in 0..128 {
            let chunk = self.to_load.pop_front();

            if chunk.is_none() {
                return;
            }

            let mut chunk = chunk.unwrap();

            Self::initialize_chunk(state, &mut chunk);

            self.active_chunks.push_back(chunk);
        }
    }

    fn create_chunks(state: &HardwareState, position: glam::IVec3, view_distance: i32, chunk_bind_group_layout: &wgpu::BindGroupLayout) -> VecDeque<Chunk> {
        let mut chunks = Vec::with_capacity(view_distance.pow(3) as usize);

        for z in -view_distance as i32..=view_distance as i32 {
            for y in -view_distance as i32..=view_distance as i32 {
                for x in -view_distance as i32..=view_distance as i32 {
                    let chunk_relative_position = glam::IVec3::new(x, y, z);
                    let chunk_position = position + chunk_relative_position;

                    if chunk_relative_position.length_squared() > view_distance.pow(2) {
                        continue;
                    }

                    let chunk = Chunk::new(state, chunk_position, chunk_bind_group_layout);
                    chunks.push(chunk);
                }
            }
        }

        chunks.sort_by_key(|chunk| chunk.position.length_squared());

        chunks.into()
    } 

    pub fn new(state: &HardwareState, view_distance: i32) -> Self {
        let chunk_bind_group_layout = Chunk::bind_group_layout(state);

        let to_load = Self::create_chunks(state, glam::IVec3::new(0, 0, 0), view_distance, &chunk_bind_group_layout);

        Self {
            position: glam::IVec3::new(0, 0, 0),
            view_distance,
            active_chunks: VecDeque::new(),
            to_load,
            to_unload: VecDeque::new(),
            chunk_bind_group_layout,
        }
    }


    pub fn visible_chunks(&self, camera: &crate::Camera) -> Vec<&Chunk> {
        let mut visible_chunks = Vec::with_capacity(self.active_chunks.len());

        for chunk in &self.active_chunks {
            if check_is_chunk_visible(chunk, camera) {
                visible_chunks.push(chunk);
            }
        }

        visible_chunks
    }


    fn _generate_chunks_to_unload(&mut self) -> VecDeque<Chunk> {
        let mut indices = Vec::new();
        let mut to_unload = VecDeque::new();

        for (index, chunk) in self.active_chunks.iter().enumerate() {
            let relative_position = chunk.position - self.position;

            if relative_position.length_squared() <= self.view_distance.pow(2) {
                continue;
            }

            indices.push(index);
        }

        for index in indices {
            let chunk = self.active_chunks.remove(index).unwrap();
            to_unload.push_back(chunk);
        }

        to_unload
    }

    fn _generate_chunks_to_load(&self) -> VecDeque<glam::IVec3> {
        todo!()
    }
}
