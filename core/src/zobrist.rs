use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::{Chessboard, BLACK, WHITE, ROOK, PAWN, Move};
use crate::r#move::MoveType;

// check https://en.wikipedia.org/wiki/Zobrist_hashing
pub struct ZobristTable {
    //board to keep track of
    board: Chessboard,
    // values for each piece and square
    piece_keys: [[[u64; 64]; 2]; 6],
    side_to_move: u64,
    castling_keys: [u64; 4],
    en_passant_keys: [u64; 16],
}

/*
        // Calculate initial Zobrist hash for the starting position
        let zobrist: &ZobristTable = &*ZOBRIST;
        self.zobrist_hash = zobrist.hash(self, WHITE);

            fn assign_en_passant(&mut self, from: u64, piece_type: u8, color: u8, zobrist: &ZobristTable) {
        // Set en_passant to the file (1–8) of the pawn that just double-pushed, or 0 for none.
        if (piece_type, color) == (PAWN, WHITE) {
            // from is on rank 1 (bits 8–15); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, file as usize);
            self.en_passant = file;
        } else if (piece_type, color) == (PAWN, BLACK) {
            // from is on rank 6 (bits 48–55); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, file as usize);
            self.en_passant = file;
        }
    }

            if new_castling != old_castling {
            // Update Zobrist hash for castling rights
            for i in 0..4 {
                let old_bit = (old_castling >> i) & 1;
                let new_bit = (new_castling >> i) & 1;
                if old_bit != new_bit {
                    zobrist.toggle_castling(&mut self.zobrist_hash, i);
                }
            }
            self.castling_rights = new_castling;
        }

        fn move_piece(&mut self, piece: u8, color: u8, mv: &Move, zobrist: &ZobristTable) {
       let from = mv.get_from_mask();
       let to = mv.get_to_mask();
       let array_index = piece_index(piece, color).unwrap();

       let from_sq = from.trailing_zeros() as usize;
       let to_sq = to.trailing_zeros() as usize;

       // XOR out the moving piece from its origin square
       zobrist.toggle_piece(&mut self.zobrist_hash, piece, color, from_sq);

       // Remove any piece on the destination square (normal capture)
       self.clear_square(to, zobrist);

       // Handle special moves
       match mv.move_type() {
           MoveType::Normal => {}
           MoveType::EnPassant { captured_square } => {
               self.clear_square(captured_square, zobrist);
           }
           MoveType::Castling { rook_from, rook_to } => {
               let rook_from_sq = rook_from.trailing_zeros() as usize;
               let rook_to_sq = rook_to.trailing_zeros() as usize;

               //update hash table
               let rook_idx = piece_index(ROOK, color).unwrap();
               zobrist.toggle_piece(&mut self.zobrist_hash, ROOK, color, rook_from_sq);
               self.pieces[rook_idx] &= !rook_from;
               self.pieces[rook_idx] |= rook_to;
               zobrist.toggle_piece(&mut self.zobrist_hash, ROOK, color, rook_to_sq);

               // Update color bitboard
               if color == WHITE {
                   self.white_pieces &= !rook_from;
                   self.white_pieces |= rook_to;
               } else {
                   self.black_pieces &= !rook_from;
                   self.black_pieces |= rook_to;
               }
           }
           MoveType::Promotion { promoted_piece } => {
               // Remove the pawn from its original square
               self.pieces[array_index] &= !from;

               // Add the promoted piece to the destination square
               let promo_index = piece_index(promoted_piece, color).unwrap();
               self.pieces[promo_index] |= to;

               // Update color occupancy for the promotion
               if color == WHITE {
                   self.white_pieces &= !from; // Remove pawn
                   self.white_pieces |= to;    // Add promoted piece
               } else {
                   self.black_pieces &= !from; // Remove pawn
                   self.black_pieces |= to;    // Add promoted piece
               }

               // Update Zobrist hash: toggle out the pawn and toggle in the promoted piece
               zobrist.toggle_piece(&mut self.zobrist_hash, PAWN, color, from_sq);
               zobrist.toggle_piece(&mut self.zobrist_hash, promoted_piece, color, to_sq);
               return; // Promotion is a special case, so we return early after handling it
           },
           MoveType::DoublePawnPush => {
               self.assign_en_passant(from, piece, color, zobrist);
           }
       }

       // XOR in the moving piece at its destination
       zobrist.toggle_piece(&mut self.zobrist_hash, piece, color, to_sq);

       // Update the piece bitboard
       self.pieces[array_index] &= !from;
       self.pieces[array_index] |= to;

       // Update color occupancy
       if color == WHITE {
           self.white_pieces &= !from;
           self.white_pieces |= to;
       } else {
           self.black_pieces &= !from;
           self.black_pieces |= to;
       }
   }

      fn clear_square(&mut self, square: u64, zobrist: &ZobristTable) {
       let sq = square.trailing_zeros() as usize;
       for p in 1..=6u8 {
           if (self.get_piece_mask(p, WHITE) & square) != 0 {
               zobrist.toggle_piece(&mut self.zobrist_hash, p, WHITE, sq);
               self.set_piece_mask(p, WHITE, self.get_piece_mask(p, WHITE) & !square);
               self.white_pieces &= !square;
           }
           if (self.get_piece_mask(p, BLACK) & square) != 0 {
               zobrist.toggle_piece(&mut self.zobrist_hash, p, BLACK, sq);
               self.set_piece_mask(p, BLACK, self.get_piece_mask(p, BLACK) & !square);
               self.black_pieces &= !square;
           }
       }
   }
 */


impl ZobristTable {
    /// create a new Zobrist table with random values
    pub fn new(board: Chessboard) -> Self {
        
        //fixed seed, maybe make test to compare seeds
        let mut rng = StdRng::seed_from_u64(69u64);

        let mut piece_keys = [[[0u64; 64]; 2]; 6];
        for piece in 0..6 {
            for color in 0..2 {
                for square in 0..64 {
                    piece_keys[piece][color][square] = rng.random();
                }
            }
        }

        let side_to_move = rng.random();

        let mut castling_keys = [0u64; 4];
        for i in 0..4 {
            castling_keys[i] = rng.random();
        }

        let mut en_passant_keys = [0u64; 16];
        for i in 0..16 {
            en_passant_keys[i] = rng.random();
        }
        
        ZobristTable {
            board,
            piece_keys,
            side_to_move,
            castling_keys,
            en_passant_keys,
        }
    }

    ///Compute the full Zobrist hash for a board position
    pub fn hash(&self, board: &Chessboard, color_to_move: u8) -> u64 {
        let mut hash = 0u64;

        // Hash all pieces using existing bitboards
        for piece_type in 1..=6u8 {  // PAWN=1 to KING=6
            let piece_idx = (piece_type - 1) as usize;

            // White pieces
            let mut white_bb = board.get_piece_mask(piece_type, WHITE);
            while white_bb != 0 {
                let sq = white_bb.trailing_zeros() as usize;
                hash ^= self.piece_keys[piece_idx][0][sq];
                white_bb &= white_bb - 1;  // Clear LSB
            }

            // Black pieces
            let mut black_bb = board.get_piece_mask(piece_type, BLACK);
            while black_bb != 0 {
                let sq = black_bb.trailing_zeros() as usize;
                hash ^= self.piece_keys[piece_idx][1][sq];
                black_bb &= black_bb - 1;
            }
        }

        //side to move
        if color_to_move == BLACK {
            hash ^= self. side_to_move;
        }

        //castling rights (4-bit flags: bit 0 black KS, bit 1 black QS, bit 2 white KS, bit 3 white QS)
        let castling = board.get_castling_rights();
        for i in 0..4 {
            if (castling >> i) & 1 != 0 {
                hash ^= self.castling_keys[i];
            }
        }

        //en passant (0 = none, otherwise a 4-bit square index)
        let ep_data = board.get_en_passant();
        if ep_data != 0 {
            let square = ep_data as usize;
            hash ^= self.en_passant_keys[square];
        }

        hash
    }

    /// Update hash when a piece is added or removed from a square, taking special moves into account
    pub(crate) fn update(mv: Move , hash: &mut u64, zobrist: &ZobristTable) {
        // Handle special moves first
        match mv.move_type() {
            MoveType::Normal => {
                    // Toggle the moving piece from its origin square
                    zobrist.toggle_piece(hash, mv.get_piece_type(), mv.get_color(), mv.get_from_mask().trailing_zeros() as usize);
                    // Toggle the moving piece at its destination
                    zobrist.toggle_piece(hash, mv.get_piece_type(), mv.get_color(), mv.get_to_mask().trailing_zeros() as usize);
            }
            MoveType::EnPassant { captured_square } => {
                zobrist.toggle_piece(hash, PAWN, if mv.get_color() == WHITE { BLACK } else { WHITE }, captured_square.trailing_zeros() as usize);
            }
            MoveType::Castling { rook_from, rook_to } => {
                zobrist.toggle_piece(hash, ROOK, mv.get_color(), rook_from.trailing_zeros() as usize);
                zobrist.toggle_piece(hash, ROOK, mv.get_color(), rook_to.trailing_zeros() as usize);
            }
            MoveType::Promotion { promoted_piece } => {
                // Toggle out the pawn and toggle in the promoted piece
                zobrist.toggle_piece(hash, PAWN, mv.get_color(), mv.get_from_mask().trailing_zeros() as usize);
                zobrist.toggle_piece(hash, promoted_piece, mv.get_color(), mv.get_to_mask().trailing_zeros() as usize);
                return; // Promotion is a special case, so we return early after handling it
            },
            MoveType::DoublePawnPush => {
                // Set en_passant to the file (1–8) of the pawn that just double-pushed, or 0 for none.
                let file = (mv.get_from_mask().trailing_zeros() % 8 + 1) as u8;
                zobrist.toggle_en_passant(hash, file as usize);
            }
        }
    }

    /// Update hash when a piece moves
    pub fn toggle_piece(&self, hash: &mut u64, piece_type: u8, color: u8, square: usize) {
        let piece_idx = (piece_type - 1) as usize;
        let color_idx = if color == WHITE { 0 } else { 1 };
        *hash ^= self.piece_keys[piece_idx][color_idx][square];
    }

    /// Toggle side to move
    pub fn toggle_side(&self, hash: &mut u64) {
        *hash ^= self.side_to_move;
    }

    /// Toggle a castling right
    pub fn toggle_castling(&self, hash: &mut u64, right_index: usize) {
        *hash ^= self.castling_keys[right_index];
    }

    /// Toggle en passant square
    pub fn toggle_en_passant(&self, hash:  &mut u64, square: usize) {
        *hash ^= self.en_passant_keys[square];
    }
}