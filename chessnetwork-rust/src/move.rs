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

    pub fn get_from_x(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        idx % 8      // Coordinate no longer flipped
    }

    pub fn get_from_y(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        idx / 8      // Coordinate no longer flipped
    }

    pub fn get_to_x(&self) -> u8 {
        let idx = self.to_mask.trailing_zeros() as u8;
        idx % 8
    }
    pub fn get_to_y(&self) -> u8 {
        let idx = self.to_mask.trailing_zeros() as u8;
        idx / 8
    }

    pub fn new_from_coordinates(from_x: u8, from_y: u8, to_x: u8, to_y: u8) -> Move {
        let from_mask = 1u64 << (from_x + from_y * 8);
        let to_mask = 1u64 << (to_x + to_y * 8);
        Move { from_mask, to_mask }
    }
}