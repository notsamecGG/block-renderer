use bitvec::prelude::*;

use crate::{Array3D, InstanceManager, HardwareState};

/// A chunk is a 16x16x16 area of blocks.
pub struct Chunk {
    // blocks: Vec<Block>,
    block_data: Array3D, // 1 block = 1 bit
    faces: Vec<BitVec>, // 6 faces = 6 bitslices
    face_count: u32,
    instance_manager: InstanceManager,
}

impl Chunk {
    pub fn new() -> Self {
        let block_data = Array3D::new(16);
        let faces = vec![BitVec::new(); 6];

        Self {
            block_data,
            faces,
            face_count: 0,
            instance_manager: InstanceManager::new(0),
        }
    }

    pub fn generate_faces(&mut self, state: &HardwareState, faces: &[BitVec; 6]) {
        let current_face_count = self.face_count;
        let mut new_face_count = 0;

        for side in faces {
            new_face_count += side.count_ones();
        }

        let new_face_count = new_face_count as u32;
        if new_face_count != current_face_count {
            self.instance_manager.set_instances(state, new_face_count as usize);
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

