use bitvec::prelude::*;

/// A chunk is a 16x16x16 area of blocks.
pub struct Chunk {
    // blocks: Vec<Block>,
    block_data: BitVec, // 1 block = 1 bit
    faces: Vec<Option<BitVec>>, // 6 faces = 6 bitslices
    face_count: u32,
}

impl Chunk {
    pub fn new() -> Self {
        let block_data = bitvec![0; 16 * 16 * 16];
        let faces = vec![None; 6];

        Self {
            block_data,
            faces,
            face_count: 0,
        }
    }
}

