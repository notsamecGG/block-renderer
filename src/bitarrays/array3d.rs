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

    pub fn to_number(&self) -> usize {
        match self {
            Self::Front => 0,
            Self::Back => 1,
            Self::Left => 2,
            Self::Right => 3,
            Self::Top => 4,
            Self::Bottom => 5,
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

    pub fn get_by_ivec(&self, pos: glam::IVec3) -> bool {
        self.get(pos.x as usize, pos.y as usize, pos.z as usize)
    }

    pub fn get_by_tuple(&self, pos: (usize, usize, usize)) -> bool {
        self.get(pos.0, pos.1, pos.2)
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
        let copy = BitVec::from_bitslice(self.data.as_bitslice());
        copy
    }

    fn reset_values_for(&self, slice: &mut BitVec, index_function: fn(usize, usize) -> usize, neighbor: &Option<BitVec>) {
        for secondary_axis in 0..16 {
            for primary_axis in 0..16 {
                let value = if let Some(neighbor) = neighbor { neighbor[secondary_axis * 16 + primary_axis] } else { false };
                slice.set(index_function(secondary_axis, primary_axis), value);
            }
        }
    }

    fn get_shifted_front(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_left(self.size.pow(2));
        
        self.reset_values_for(slice, |y, x| 16 * 16 * 15 + y * 16 + x, neighbor);
    }

    fn get_shifted_back(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_right(self.size.pow(2));
        
        self.reset_values_for(slice, |y, x| y * 16 + x, neighbor);
    }

    fn get_shifted_top(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_left(self.size);

        self.reset_values_for(slice, |z, x| 16 * 15 + z * 256 + x, neighbor);
    }

    fn get_shifted_bottom(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_right(self.size);

        self.reset_values_for(slice, |z, x| z * 256 + x, neighbor);
    }

    fn get_shifted_left(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_left(1);

        self.reset_values_for(slice, |z, y| z * 256 + y * 16 + 15, neighbor);
    }
    
    fn get_shifted_right(&self, slice: &mut BitVec, neighbor: &Option<BitVec>) {
        slice.shift_right(1);
        
        self.reset_values_for(slice, |z, y| z * 256 + y * 16, neighbor);
    }

    pub fn get_shifted(&self, shift_direction: &ShiftDirection, neighbors: &Vec<Option<BitVec>>) -> BitVec {
        if let Some(slice) = &self.shifted[shift_direction.to_number()] {
            // todo: fix this check
            if slice == &self.data {
                return slice.clone();
            }
        }

        let mut slice = self.copy_bitvec();
        match shift_direction {
            ShiftDirection::Front  => self.get_shifted_front (&mut slice, &neighbors[0]),
            ShiftDirection::Back   => self.get_shifted_back  (&mut slice, &neighbors[1]),
            ShiftDirection::Left   => self.get_shifted_left  (&mut slice, &neighbors[2]),
            ShiftDirection::Right  => self.get_shifted_right (&mut slice, &neighbors[3]),
            ShiftDirection::Top    => self.get_shifted_top   (&mut slice, &neighbors[4]),
            ShiftDirection::Bottom => self.get_shifted_bottom(&mut slice, &neighbors[5]),
        }
        
        slice
    }

    pub fn compare_shifted(&self, shift_direction: ShiftDirection, neighbors: &Vec<Option<BitVec>>) -> BitVec {
        let mut shifted = self.get_shifted(&shift_direction, neighbors);
        shifted.bitxor_assign(&self.data);
        shifted.bitand_assign(&self.data);
        shifted
    }

    pub fn get_faces(&self, neighbors: Vec<Option<BitVec>>) -> [BitVec; 6] {
        [
            self.compare_shifted(ShiftDirection::Front, &neighbors),
            self.compare_shifted(ShiftDirection::Back, &neighbors),
            self.compare_shifted(ShiftDirection::Left, &neighbors),
            self.compare_shifted(ShiftDirection::Right, &neighbors),
            self.compare_shifted(ShiftDirection::Top, &neighbors),
            self.compare_shifted(ShiftDirection::Bottom, &neighbors),
        ]
    }
}

