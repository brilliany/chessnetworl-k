use serde::{Serialize, Deserialize};

/// Special move types including corresponding data
#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveType {
    #[default]
    Normal,
    /// En passant capture captured_square is the mask of the pawn being taken
    /// (the pawn behind the destination square)
    EnPassant { captured_square: u64 },
    /// Castling. rook_from and rook_to is a rook move 
    Castling { rook_from: u64, rook_to: u64 },
    // Future: Promotion { promote_to: u8 },
}

/// from_mask and to_mask are u64s with the relevant bit flipped to a 1, they are stored this way
/// as opposed to coordinates or square indexes for consistency across the project.
#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Move {
    from_mask: u64,
    to_mask: u64,
    kind: MoveType,
}

impl Move {
    /// Standard move
    pub fn new(from_mask: u64, to_mask: u64) -> Move {
        Move { from_mask, to_mask, kind: MoveType::Normal }
    }

    /// En passant capture captured_square is the mask of the enemy pawn being removed.
    pub fn en_passant(from_mask: u64, to_mask: u64, captured_square: u64) -> Move {
        Move {
            from_mask,
            to_mask,
            kind: MoveType::EnPassant { captured_square },
        }
    }

    /// Castling move king moves from from_mask to to_mask and rook moves from rook_from to rook_to
    pub fn castling(from_mask: u64, to_mask: u64, rook_from: u64, rook_to: u64) -> Move {
        Move {
            from_mask,
            to_mask,
            kind: MoveType::Castling { rook_from, rook_to },
        }
    }
    
    ///Would be nice just to call this 'type' wouldnt it
    pub fn mv_type(&self) -> MoveType {
        self.kind
    }

    pub fn get_from_mask(&self) -> u64 {
        self.from_mask
    }
    pub fn get_to_mask(&self) -> u64 {
        self.to_mask
    }

    pub fn get_from_x(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        idx % 8
    }
    pub fn get_from_y(&self) -> u8 {
        let idx = self.from_mask.trailing_zeros() as u8;
        idx / 8
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
        Move { from_mask, to_mask, kind: MoveType::Normal }
    }
}