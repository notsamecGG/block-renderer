use std::collections::{VecDeque, HashMap};
use bitvec::prelude::*;

use crate::{Chunk, HardwareState, ShiftDirection};


fn check_is_chunk_visible(_chunk: &Chunk, _camera: &crate::Camera) -> bool {
    true
}

pub struct ChunkManager {
    _position: glam::IVec3,
    _view_distance: i32,

    chunk_positions: HashMap<glam::IVec3, Chunk>,
    chunk_bind_group_layout: wgpu::BindGroupLayout,

    active_chunks: VecDeque<glam::IVec3>,
    to_load: VecDeque<glam::IVec3>,
    _to_unload: VecDeque<glam::IVec3>,
}

impl ChunkManager {
    pub fn chunk_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.chunk_bind_group_layout
    }
}

impl ChunkManager {
    /// Gets data of each neighboring face of the chunk
    fn get_neighbors(&self, chunk_position: glam::IVec3) -> Vec<Option<BitVec>> {
        let mut neighbors = Vec::with_capacity(6);

        for i in 0..6 {
            let neighbor_direction = ShiftDirection::from_number(i);
            let neighbor_relative_pos = match neighbor_direction {
                ShiftDirection::Front => glam::IVec3::Z,
                ShiftDirection::Back => glam::IVec3::NEG_Z,
                ShiftDirection::Left => glam::IVec3::X,
                ShiftDirection::Right => glam::IVec3::NEG_X,
                ShiftDirection::Top => glam::IVec3::Y,
                ShiftDirection::Bottom => glam::IVec3::NEG_Y,
            };

            let neighbor_pos = chunk_position + neighbor_relative_pos;
            let neighbor = self.chunk_positions.get(&neighbor_pos);

            if let Some(neighbor) = neighbor {
                let mut neighbor_face = BitVec::with_capacity(16 * 16);

                for secondary_axis in 0..16 {
                    for primary_axis in 0..16 {
                        let neighbor_face_bit = match neighbor_direction {
                            ShiftDirection::Front  => neighbor.block_data.get(primary_axis, secondary_axis, 15),
                            ShiftDirection::Back   => neighbor.block_data.get(primary_axis, secondary_axis, 15),
                            ShiftDirection::Top    => neighbor.block_data.get(primary_axis, 15, secondary_axis),
                            ShiftDirection::Bottom => neighbor.block_data.get(primary_axis, 15, secondary_axis),
                            ShiftDirection::Left   => neighbor.block_data.get(15, primary_axis, secondary_axis),
                            ShiftDirection::Right  => neighbor.block_data.get(15, primary_axis, secondary_axis),
                        };

                        neighbor_face.push(neighbor_face_bit);
                    }
                }

                neighbors.push(Some(neighbor_face));
            } else {
                neighbors.push(None);
            }
        }

        neighbors
    }

    fn initialize_chunk(chunk: &mut Chunk) {
        // chunk.block_data.set(8, 8, 8, true);

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    if x == 0 || y == 0 || x == 15 || y == 15 {
                        chunk.block_data.set(x, y, z, true);
                    }
                }
            }
        }
    }

    fn update_faces(state: &HardwareState, chunk: &mut Chunk, neighbors: Vec<Option<BitVec>>) {
        chunk.update_faces(&state, neighbors);
    }

    pub fn initialize_chunks(&mut self, state: &HardwareState) {
        if self.to_load.is_empty() {
            return;
        }

        for chunk_position in self.to_load.iter() {
            let chunk = self.chunk_positions.get_mut(&chunk_position).unwrap();

            Self::initialize_chunk(chunk);
        }

        for _ in 0..128 {
            let chunk_position = self.to_load.pop_front();

            if chunk_position.is_none() {
                return;
            }

            let chunk_position = chunk_position.unwrap();
            let neighbors = self.get_neighbors(chunk_position);
            let chunk = self.chunk_positions.get_mut(&chunk_position).unwrap();

            Self::initialize_chunk(chunk);
            Self::update_faces(state, chunk, neighbors);

            self.active_chunks.push_back(chunk_position);
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

        let chunks = Self::create_chunks(state, glam::IVec3::new(0, 0, 0), view_distance, &chunk_bind_group_layout);
        let to_load = chunks.iter().map(|chunk| chunk.position).collect();
        let chunk_positions = chunks.into_iter().map(|chunk| (chunk.position, chunk));
        let chunk_positions = HashMap::from_iter(chunk_positions);

        Self {
            _position: glam::IVec3::new(0, 0, 0),
            _view_distance: view_distance,
            chunk_positions,
            active_chunks: VecDeque::new(),
            to_load,
            _to_unload: VecDeque::new(),
            chunk_bind_group_layout,
        }
    }


    pub fn visible_chunks(&self, camera: &crate::Camera) -> Vec<&Chunk> {
        let mut visible_chunks = Vec::with_capacity(self.active_chunks.len());

        for chunk_position in &self.active_chunks {
            let chunk = self.get_chunk(chunk_position);
            if check_is_chunk_visible(chunk, camera) && chunk.face_count() != 0 {
                visible_chunks.push(chunk);
            }
        }

        visible_chunks
    }

    fn get_chunk(&self, chunk_position: &glam::IVec3) -> &Chunk {
        self.chunk_positions.get(chunk_position).unwrap()
    }

    fn _get_chunk_mut(&mut self, chunk_position: &glam::IVec3) -> &mut Chunk {
        self.chunk_positions.get_mut(chunk_position).unwrap()
    }
}
