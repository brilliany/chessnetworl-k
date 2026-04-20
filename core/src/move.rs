use serde::{Serialize, Deserialize};
use crate::{BLACK, WHITE};

/// Special move types including corresponding data
#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveType {
    #[default]
    Normal,
    DoublePawnPush,
    /// En passant capture captured_square is the mask of the pawn being taken
    /// (the pawn behind the destination square)
    EnPassant { captured_square: u64 },
    /// Castling. rook_from and rook_to is a rook move 
    Castling { rook_from: u64, rook_to: u64 },
    // Promotion. promoted_piece is the piece type the pawn is promoted to
    Promotion { promoted_piece: u8 },
}

/// Compact move representation using bit manipulation. The move is represented as a 16-bit integer where:
/// - Bits 0-5: from square (0-63)
/// - Bits 6-11: to square (0-63)
/// - Bits 12-14: piece type, this is stored to avoid looking up the piece type on move even though it could be implied
/// - Bits 15: piece color, same reason
#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Move {
    move_representation: u16,
    move_type: MoveType,
}

impl Move {
    /// Standard move
    pub fn new(from_mask: u64, to_mask: u64, piece_type: u8, color: u8) -> Move {
        let move_representation =
            (from_mask.trailing_zeros() as u16) // from square in bits 0-5
            | ((to_mask.trailing_zeros() as u16) << 6) // to square in bits 6-11
            | ((piece_type as u16) << 12) // piece type in bits 12-14
            | ((color as u16) << 15); // color in bit 15

        Move { move_representation, move_type: MoveType::Normal }
    }
    
    pub fn double_pawn_push(from_mask: u64, to_mask: u64, piece_type: u8, color: u8) -> Move {
        let move_representation = Move::new(from_mask, to_mask, piece_type, color).move_representation;
        Move { move_representation, move_type: MoveType::DoublePawnPush }
    }

    /// En passant capture captured_square is the mask of the enemy pawn being removed.
    pub fn en_passant(from_mask: u64, to_mask: u64, piece_type: u8, color: u8, captured_square: u64) -> Move {
        let move_representation = Move::new(from_mask, to_mask, piece_type, color).move_representation;
        Move {
            move_representation,
            move_type: MoveType::EnPassant { captured_square },
        }
    }

    /// Castling move king moves from from_mask to to_mask and rook moves from rook_from to rook_to
    pub fn castling(from_mask: u64, to_mask: u64, piece_type: u8, color: u8, rook_from: u64, rook_to: u64) -> Move {
        let move_representation = Move::new(from_mask, to_mask, piece_type, color).move_representation;
        Move {
            move_representation,
            move_type: MoveType::Castling { rook_from, rook_to },
        }
    }

    pub fn promotion(from_mask: u64, to_mask: u64, piece_type: u8, color: u8, promoted_piece: u8) -> Move {
        let move_representation = Move::new(from_mask, to_mask, piece_type, color).move_representation;
        Move {
            move_representation,
            move_type: MoveType::Promotion { promoted_piece },
        }
    }
    
    ///Would be nice just to call this 'type' wouldnt it
    pub fn move_type(&self) -> MoveType {
        self.move_type
    }

    pub fn is_en_passant(&self) -> bool {
        matches!(self.move_type, MoveType::EnPassant { .. })
    }

    pub fn is_castling(&self) -> bool {
        matches!(self.move_type, MoveType::Castling { .. })
    }

    pub fn get_from_mask(&self) -> u64 {
        1u64 << (self.move_representation & 0b111111) // bits 0-5 for from square
    }
    pub fn get_to_mask(&self) -> u64 {
        1u64 << ((self.move_representation >> 6) & 0b111111) // bits 6-11 for to square
    }
    
    pub fn get_piece_type(&self) -> u8 {
        ((self.move_representation >> 12) & 0b111) as u8 // bits 12-14 for piece type
    }
    
    pub fn get_color(&self) -> u8 {
        if (self.move_representation >> 15) & 1 == 1 {
            WHITE
        } else {
            BLACK
        }
    }
    
    pub fn get_from_idx(&self) -> u8 {
        (self.move_representation & 0b111111) as u8 
    }
    
    pub fn get_to_idx(&self) -> u8 {
        ((self.move_representation >> 6) & 0b111111) as u8
    }
}