use chessnetwork_derives::{EnumName, FromI32};

#[derive(Copy, Clone, EnumName, FromI32, Debug)]
#[derive(PartialEq)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
    Empty = 0,
}


#[derive(Copy, Clone, EnumName, FromI32)]
pub enum Color {
    White = 1,
    Black = -1,
    None = 0,
}



pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Piece {
        Piece { piece_type, color }
    }
    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn to_string(&self) -> String {
        format!("{}_{:?}", self.color.name(), self.piece_type.name())
    }
}


