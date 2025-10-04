use serde::{Serialize, Deserialize};

#[derive(Default)]
#[derive(Copy, Clone, Debug)]
#[derive(Hash, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Move {
    bits: u16,
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
impl Move {
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
}
