use serde::{Serialize, Deserialize};

#[derive(Default)]
#[derive(Copy, Clone, Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Move {
    from_mask: u64,
    to_mask: u64,
}
/*
todo the memory 'density' isnt that important since we never store that many moves in memory at once
todo just store the moves as bit masks for the board to make move generation faster 
*/
/**
    * first 4 bits: from square x
    * second 4 bits: from square y
    * third 4 bits: to square x
    * fourth 4 bits: to square y
    */
/*impl Move {
    pub fn new(from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> Move {
        let mut bits: u16 = 0;
        bits |= from_x as u16;
        bits |= (from_y as u16) << 4;
        bits |= (to_x as u16) << 8;
        bits |= (to_y as u16) << 12;
        Move { bits }
    }
    pub fn get_from_x(&self) -> u8 {
        (self.bits & 0b1111) as u8
    }
    pub fn get_from_y(&self) -> u8 {
        ((self.bits >> 4) & 0b1111) as u8
    }
    pub fn get_to_x(&self) -> u8 {
        ((self.bits >> 8) & 0b1111) as u8
    }
    pub fn get_to_y(&self) -> u8 {
        ((self.bits >> 12) & 0b1111) as u8
    }
}*/

impl Move {
    pub fn new(from_mask: u64, to_mask: u64) -> Move {
        Move { from_mask, to_mask }
    }
    pub fn get_from_mask(&self) -> u64 {
        self.from_mask
    }
    pub fn get_to_mask(&self) -> u64 {
        self.to_mask
    }

    // slow coordinate getters for front e.g frontend (flipped: origin at bottom-right)
    pub fn get_from_x(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        let x = idx % 8;
        7 - x
    }
    pub fn get_from_y(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        let y = idx / 8;
        7 - y
    }
    pub fn get_to_x(&self) -> u8 {
        let idx = self.to_mask.trailing_zeros() as u8;
        let x = idx % 8;
        7 - x
    }
    pub fn get_to_y(&self) -> u8 {
        let idx = self.to_mask.trailing_zeros() as u8;
        let y = idx / 8;
        7 - y
    }

    // Accepts frontend (flipped) coordinates and converts them to internal bitmask indices
    pub fn new_from_coordinates(from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> Move {
        // convert frontend coords (flipped origin at bottom-right) to internal (origin top-left)
        let fx = 7 - from_x;
        let fy = 7 - from_y;
        let tx = 7 - to_x;
        let ty = 7 - to_y;

        let from_mask = 1u64 << (fx + fy * 8);
        let to_mask = 1u64 << (tx + ty * 8);
        Move { from_mask, to_mask }
    }
}
