use crate::*;
use crate::r#move::{Move, MoveType};
use rand:: {Rng, SeedableRng};
use rand:: rngs::StdRng;

use lazy_static::lazy_static;

//once values are initialized we dont need to make new tables
lazy_static! {
    pub static ref ZOBRIST:  ZobristTable = ZobristTable::new();
}


/// Lightweight version of board state for the undo stack
/// Avoids cloning the full `Chessboard` (and its history Vec) on every move
#[derive(Clone, Debug)]
pub struct BoardState {
    pieces: [u64; 12],
    white_pieces: u64,
    black_pieces: u64,
    castling_rights: u8,
    en_passant: u8,
    zobrist_hash: u64,
}

#[derive(Default, Debug)]
#[derive(Clone)]
pub struct Chessboard {
    //pieces stored in bitboards
    pieces: [u64; 12], //array of bitboards for each piece type, index 0-5 for white pieces, 6-11 for black pieces
    //order: pawn, knight, bishop, rook, queen, king

    white_pieces: u64, //bitboard for all white pieces

    black_pieces: u64, //bitboard for all black pieces

    /// Castling rights as 4-bit flags:
    /// bit 0: black kingside, bit 1: black queenside,
    /// bit 2: white kingside, bit 3: white queenside
    castling_rights: u8,

    /// En passant target file, stored as a 1-based index [1–8], or 0 for none.
    /// The same encoding is used regardless of which color just double-pushed.
    /// The move generator derives the correct rank from context:
    ///   - White captures, moving to rank 5, capturing pawn on rank 4: bit index = (file-1) + 4*8
    ///   - Black captures, moving to rank 2, capturing pawn on rank 3: bit index = (file-1) + 3*8
    en_passant: u8,

    //history stack for undoing moves
    history: Vec<BoardState>,

    zobrist_hash: u64,
}

impl Chessboard {
    // getters and setters for pieces and castling rights
    pub fn get_piece_mask(&self, piece: u8, color: i8) -> u64 {
        self.pieces[piece_index(piece, color).unwrap()].clone()
    }

    /// set the bitboard for a specific piece type and color NOTE: does not update white_pieces or black_pieces bitboards
    pub fn set_piece_mask(&mut self, piece: u8, color: i8, mask: u64) {
        self.pieces[piece_index(piece, color).unwrap()] = mask;
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_pieces.clone()
    }
    pub fn set_white_pieces(&mut self, white_pieces: u64) {
        self.white_pieces = white_pieces;
    }
    pub fn get_black_pieces(&self) -> u64 {
        self.black_pieces.clone()
    }
    pub fn set_black_pieces(&mut self, black_pieces: u64) {
        self.black_pieces = black_pieces;
    }

    pub fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }
    pub fn set_castling_rights(&mut self, rights: u8) {
        self.castling_rights = rights;
    }
    pub fn get_en_passant(&self) -> u8 {
        self.en_passant
    }
    pub fn set_en_passant(&mut self, ep: u8) {
        self.en_passant = ep;
    }

    pub fn get_history(&self) -> &Vec<BoardState> {
        &self.history
    }
    pub fn get_hash(&self) -> u64 {
        self.zobrist_hash.clone()
    }

    pub fn init(&mut self) {
        //todo possibly add support for more complex starting positions later, maybe loaded from a file
        //clear all bitboards
        self.pieces = [0; 12];
        self.white_pieces = 0;
        self.black_pieces = 0;
        self.history = Vec::new();

        // Apply placements
        for &(ptype, color, mask) in STARTING_POS {
            self.set_piece_mask(ptype, color, mask);
            if color == WHITE {
                self.white_pieces |= mask;
            } else {
                self.black_pieces |= mask;
            }
        }
        self.set_castling_rights(0b1111); // all castling rights available
        self.set_en_passant(0);           // no en passant
    }

    pub fn get_piece_at(&self, pos: u64) -> (u8, i8) {
        for piece in 1..=6 {
            if (self.get_piece_mask(piece, WHITE) & pos) != 0 {
                return (piece, WHITE);
            }
            if (self.get_piece_mask(piece, BLACK) & pos) != 0 {
                return (piece, BLACK);
            }
        }
        (EMPTY, NONE)
    }

    pub fn print_board(&self) {
        println!();
        for i in 0..8 {
            print!("{}|", 8 - i);
            for j in 0..8 {
                let piece = self.get_piece_at(1u64 << (7-j + (7-i) * 8));
                let piece_type = piece.0;
                if piece_type == EMPTY {
                    print!("  |");
                } else {
                    if piece.1 < 0 {
                        print!("-{:?}|", piece_type);
                    } else {
                        print!(" {:?}|", piece_type);
                    }
                }
            }
            println!();
        }
        println!("   A  B  C  D  E  F  G  H");

    }

    /// Make a move and add the current state of the board to the history stack
    pub fn make_move(&mut self, mv: Move) {
        let zobrist: &ZobristTable = &*ZOBRIST;
        // Save current state to history
        self.history.push(BoardState {
            pieces: self.pieces,
            white_pieces: self.white_pieces,
            black_pieces: self.black_pieces,
            castling_rights: self.castling_rights,
            en_passant: self.en_passant,
            zobrist_hash: self.zobrist_hash,
        });

        let from = mv.get_from_mask();
        let to = mv.get_to_mask();
        let piece = self.get_piece_at(from);
        let (piece_type, color) = piece;

        // If en passant is active, XOR it out of the hash before clearing it
        if self.en_passant != 0 {
            zobrist.toggle_en_passant(&mut self.zobrist_hash, self.en_passant as usize);
        }
        self.en_passant = 0;

        // Update castling rights
        self.update_castling_rights(from, to, zobrist);
        self.assign_en_passant(from, to, piece_type, color, zobrist);

       self.move_piece(piece_type, color, &mv, zobrist);

       // Toggle side to move
       zobrist.toggle_side(&mut self.zobrist_hash);

   }

    fn assign_en_passant(&mut self, from: u64, to: u64, piece_type: u8, color: i8, zobrist: &ZobristTable) {
        // Set en_passant to the file (1–8) of the pawn that just double-pushed, or 0 for none.
        // Using 1-based file so that 0 unambiguously means "no en passant".
        if (piece_type, color) == (PAWN, WHITE) && from << 16 == to {
            // from is on rank 1 (bits 8–15); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, file as usize);
            self.en_passant = file;
        } else if (piece_type, color) == (PAWN, BLACK) && from >> 16 == to {
            // from is on rank 6 (bits 48–55); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, file as usize);
            self.en_passant = file;
        }
    }

    fn update_castling_rights(&mut self, from: u64, to: u64, zobrist: &ZobristTable) {
        let old_castling = self.castling_rights;
        let mut new_castling = old_castling;

        // White King moved (e1)
        if (from & (1u64 << 4)) != 0 {
            new_castling &= !0b1100;
        }
        // Black King moved (e8)
        if (from & (1u64 << 60)) != 0 {
            new_castling &= !0b0011;
        }

        // Check if rooks are moved or captured (a1, h1, a8, h8)

        // White Queenside Rook (a1)
        if (from & 1u64) != 0 || (to & 1u64) != 0 {
            new_castling &= !0b1000;
        }
        // White Kingside Rook (h1)
        if (from & (1u64 << 7)) != 0 || (to & (1u64 << 7)) != 0 {
            new_castling &= !0b0100;
        }
        // Black Queenside Rook (a8)
        if (from & (1u64 << 56)) != 0 || (to & (1u64 << 56)) != 0 {
            new_castling &= !0b0010;
        }
        // Black Kingside Rook (h8)
        if (from & (1u64 << 63)) != 0 || (to & (1u64 << 63)) != 0 {
            new_castling &= !0b0001;
        }

        if new_castling != old_castling {
             for i in 0..4 {
                if ((old_castling >> i) & 1) != ((new_castling >> i) & 1) {
                    zobrist.toggle_castling(&mut self.zobrist_hash, i);
                }
             }
             self.castling_rights = new_castling;
        }
    }

    /**
     Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
     The move_piece and remove_piece functions assume that the move is valid and legal
    */
   //todo promotion
   fn move_piece(&mut self, piece: u8, color: i8, mv: &Move, zobrist: &ZobristTable) {
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
       match mv.mv_type() {
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

   /// Undo the last move by restoring the previous board state
   pub fn undo_move(&mut self) {
       if let Some(state) = self.history.pop() {
           self.pieces = state.pieces;
           self.white_pieces = state.white_pieces;
           self.black_pieces = state.black_pieces;
           self.castling_rights = state.castling_rights;
           self.en_passant = state.en_passant;
           self.zobrist_hash = state.zobrist_hash;
       } else {
           panic!("Cannot undo move: history is empty");
       }
   }

   ///Returns the current board state as a JSON string, flipped since this is only used for the frontend where white is on the bottom
/* TODO replace with serde, will break frontend
{
   *   board: [
   *       [black_rook, black_knight, black_bishop, black_queen, black_king, black_bishop, black_knight, black_rook],
   *       [black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn],
   *       [empty, empty, empty, empty, empty, empty, empty, empty],
   *       [empty, empty, empty, empty, empty, empty, empty, empty],
   *       [empty, empty, empty, empty, empty, empty, empty, empty],
   *       [empty, empty, empty, empty, empty, empty, empty, empty],
   *       [empty, empty, empty, empty, empty, empty, empty, empty],
   *       [white_pawn, white_pawn, white_pawn, white_pawn, white_pawn, white_pawn, white_pawn, white_pawn],
   *       [white_rook, white_knight, white_bishop, white_queen, white_king, white_bishop, white_knight, white_rook],
   *    ]
   * }
*/
   pub fn convert_to_json(&mut self) -> String {
       let mut json = String::new();
       json.push_str("{\n");
       json.push_str("\t\"board\": [\n");
       for rank in 0..8 {
           json.push_str("\t\t[");
           for file in 0..8 {
               let square = file + rank * 8;
               let square_mask: u64 = 1u64 << square;
               let (piece_type, color) = self.get_piece_at(square_mask);
               let piece_value = match (piece_type, color) {
                   (PAWN, WHITE) => "white_pawn",
                   (KNIGHT, WHITE) => "white_knight",
                   (BISHOP, WHITE) => "white_bishop",
                   (ROOK, WHITE) => "white_rook",
                   (QUEEN, WHITE) => "white_queen",
                   (KING, WHITE) => "white_king",
                   (PAWN, BLACK) => "black_pawn",
                   (KNIGHT, BLACK) => "black_knight",
                   (BISHOP, BLACK) => "black_bishop",
                   (ROOK, BLACK) => "black_rook",
                   (QUEEN, BLACK) => "black_queen",
                   (KING, BLACK) => "black_king",
                   (EMPTY, NONE) => "empty",
                   _ => "invalid",
               };
               json.push_str("\"");
               json.push_str(piece_value);
               json.push_str("\"");
               if file != 7 {
                   json.push_str(", ");
               }
           }
           json.push_str("]");
           if rank != 7 {
               json.push_str(",\n"); // <-- Change 7 to ensure JSON ends properly
           } else {
               json.push_str("\n");
           }
       }
       json.push_str("\t]\n");
       json.push_str("}\n");
       json
   }
}

/// helper: map piece (1..6) and color (WHITE/BLACK) to pieces[] index (0..11)
fn piece_index(piece: u8, color: i8) -> Option<usize> {
   // a lot of functions depend on this, so panic on invalid input
   if piece == 0 || piece > 6 {
       /*self.print_history();*/
       panic!("Invalid piece type: {},", piece);
   }
   if color == WHITE {
       Some((piece - 1) as usize)
   } else if color == BLACK {
       Some((piece + 5) as usize)
   } else {
       panic!("Invalid color");
   }
}


// check https://en.wikipedia.org/wiki/Zobrist_hashing
pub struct ZobristTable {
   // values for each piece and square
   piece_keys: [[[u64; 64]; 2]; 6],
   side_to_move: u64,
   castling_keys: [u64; 4],
   en_passant_keys: [u64; 16],
}


impl ZobristTable {
    /// create a new Zobrist table with random values
    pub fn new() -> Self {
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
            piece_keys,
            side_to_move,
            castling_keys,
            en_passant_keys,
        }
    }

    ///Compute the full Zobrist hash for a board position
    pub fn hash(&self, board: &Chessboard, side_to_move: i8) -> u64 {
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
        if side_to_move == BLACK {
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

    /// Update hash when a piece moves
    pub fn toggle_piece(&self, hash: &mut u64, piece_type: u8, color: i8, square: usize) {
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

// util function
pub fn print_bitboard_as_chessboard(board: u64) {
    for y in 0..8 {
        for x in 0..8 {
            let bit = 1 << (x + y * 8);
            if (board & bit) != 0 {
                print!("|1");
            } else {
                print!("|0");
            }
        }
        println!();
    }
}