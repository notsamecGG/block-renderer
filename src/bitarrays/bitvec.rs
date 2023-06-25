use crate::Axis;

/// Treats the inside data as an array of singular bits or groups of them 
/// with a group size given by group_len (defaults to 1 bit).
pub struct BitVec {
    data: Vec<u64>,
    group_len: usize,
    len: usize,
    basic_mask: u64,
}

impl BitVec {
    const CHUNK_LEN: usize = 64;

    /// get the mask for a given size (in bits), starting from the left
    fn create_mask(size: usize, start: usize) -> u64 {
        !(std::u64::MAX >> (size)) >> start
    }

    pub fn new(group_len: usize) -> Self {
        Self {
            data: Vec::new(),
            group_len,
            len: 0,
            basic_mask: Self::create_mask(1, 0),
        }
    }

    pub fn get_group(&self, index: usize, base: usize) -> u64 {
        let chunk_index = index / Self::CHUNK_LEN;
        let group_start = index % Self::CHUNK_LEN;

        let chunk = self.data[chunk_index];
        let mask = self.basic_mask >> group_start;

        (chunk & mask) << group_start >> base
    }

    pub fn set_group(&mut self, index: usize, value: u64) {
        let chunk_index = index / Self::CHUNK_LEN;
        let group_start = index % Self::CHUNK_LEN;

        let chunk = self.data[chunk_index];
        let mask = self.basic_mask >> group_start;

        let value = value << (64 - 1 - group_start);

        self.data[chunk_index] = (chunk & !mask) | value;
    }

    pub fn get_multiple_groups(&self, start: usize, count: usize) -> u64 {
        let mut result = 0;

        for i in 0..count {
            result |= self.get_group(start + i, self.group_len * i);
        }

        result
    }
}
