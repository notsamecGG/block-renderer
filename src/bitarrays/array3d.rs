use std::ops::{BitXorAssign, BitAndAssign};

// use deku::prelude::*;
use bitvec::prelude::*;


pub enum ShiftDirection {
    Front = 0,
    Back = 1,
    Left = 2,
    Right = 3,
    Top = 4,
    Bottom = 5,
}

impl ShiftDirection {
    pub fn from_number(number: usize) -> Self {
        match number {
            0 => Self::Front,
            1 => Self::Back,
            2 => Self::Left,
            3 => Self::Right,
            4 => Self::Top,
            5 => Self::Bottom,
            _ => panic!("Invalid shift direction: {}", number),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Array3D {
    data: BitVec,
    size: usize,

    shifted: Vec<Option<BitVec>>,
}

impl Array3D {
    /// Creates a new 3D array with the given size.
    /// Size is the length of one side of the array cube.
    pub fn new(size: usize) -> Self {
        let data = bitvec![0; size.pow(3)];
        let shifted = vec![None; 6];

        Self {
            data,
            size,
            shifted
        }
    }

    fn get_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.size + z * self.size.pow(2)
    }

    pub fn data_mut(&mut self) -> &mut BitSlice {
        self.data.as_mut_bitslice()
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= self.size || y >= self.size || z >= self.size {
            panic!("Index out of bounds: ({}, {}, {})", x, y, z);
        }

        let index = self.get_index(x, y, z);
        self.data[index]
    }

    // todo: handle group_len 
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: bool) {
        if x >= self.size || y >= self.size || z >= self.size {
            panic!("Index out of bounds: ({}, {}, {})", x, y, z);
        }

        let index = self.get_index(x, y, z);
        self.data.set(index, value);
    }

    fn copy_bitvec(&self) -> BitVec {
        let mut copy = BitVec::new();
        copy.copy_from_bitslice(self.data.as_bitslice());
        copy
    }


    fn get_shifted_front(&self, slice: &mut BitVec) {
        slice.shift_left(self.size.pow(2));
    }

    fn get_shifted_back(&self, slice: &mut BitVec) {
        slice.shift_right(self.size.pow(2));
    }

    fn get_shifted_top(&self, slice: &mut BitVec) {
        slice.shift_left(self.size);
    }

    fn get_shifted_bottom(&self, slice: &mut BitVec) {
        slice.shift_right(self.size);
    }

    fn get_shifted_left(&self, slice: &mut BitVec) {
        slice.shift_right(1);
    }
    
    fn get_shifted_right(&self, slice: &mut BitVec) {
        slice.shift_left(1);
    }

    pub fn get_shifted(&self, shift_direction: ShiftDirection) -> BitVec {
        if let Some(slice) = &self.shifted[shift_direction as usize] {
            if slice == &self.data {
                return slice.clone();
            }
        }

        let mut slice = self.copy_bitvec();
        match shift_direction {
            ShiftDirection::Front  => self.get_shifted_front(&mut slice),
            ShiftDirection::Back   => self.get_shifted_back(&mut slice),
            ShiftDirection::Left   => self.get_shifted_left(&mut slice),
            ShiftDirection::Right  => self.get_shifted_right(&mut slice),
            ShiftDirection::Top    => self.get_shifted_top(&mut slice),
            ShiftDirection::Bottom => self.get_shifted_bottom(&mut slice),
        }
        
        slice
    }

    pub fn compare_shifted(&self, shift_direction: ShiftDirection) -> BitVec {
        let mut shifted = self.get_shifted(shift_direction);
        shifted.bitxor_assign(&self.data);
        shifted.bitand_assign(&self.data);
        shifted
    }

    pub fn get_faces(&self) -> [BitVec; 6] {
        [
            self.compare_shifted(ShiftDirection::Front),
            self.compare_shifted(ShiftDirection::Back),
            self.compare_shifted(ShiftDirection::Left),
            self.compare_shifted(ShiftDirection::Right),
            self.compare_shifted(ShiftDirection::Top),
            self.compare_shifted(ShiftDirection::Bottom),
        ]
    }
}

