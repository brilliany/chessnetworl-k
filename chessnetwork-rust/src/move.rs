use serde::{Serialize, Deserialize};
use crate::print_bitboard_as_chessboard;

#[derive(Default)]
#[derive(Copy, Clone, Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Move {
    from_mask: u64,
    to_mask: u64,
}
/**
from_mask and to_mask are u64s with the relevant bit flipped to a 1, they are stored this way as opposed to coordinates or square indexes for consistency across the project
*/
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

    /// Accepts frontend (flipped) coordinates and converts them to internal bitmask indices
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
