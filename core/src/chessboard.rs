use crate::*;
use crate::r#move::Move;
use rand:: {Rng, SeedableRng};
use rand:: rngs::StdRng;

use lazy_static::lazy_static;

//once values are initialized we dont need to make new tables
lazy_static! {
    pub static ref ZOBRIST:  ZobristTable = ZobristTable::new();
}


#[derive(Default, Debug)]
#[derive(Clone)]
pub struct Chessboard {
    //pieces stored in bitboards
    pieces: [u64; 12], //array of bitboards for each piece type, index 0-5 for white pieces, 6-11 for black pieces
    //order: pawn, knight, bishop, rook, queen, king

    white_pieces: u64, //bitboard for all white pieces

    black_pieces: u64, //bitboard for all black pieces

    /**

    castling rights stored in bit flags, 1 for right, 0 for no right
    first bit: black kingside, second bit: black queenside, third bit: white kingside, fourth bit: white queenside

    castling in 4 bits starting from top left (0, 0), just a bit flag for if castling is available for that corner
    en passant is the remaining 4 bits, read as a 4 bit number 0-15, indexing a square on the 2 ranks where en passant is possible

     */
    /*
    en passant squares:
    A B C D E F G H
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    x x x x x x x x
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    x x x x x x x x
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    */
    castling_en_passant: u8,
    
    //history stack for undoing moves
    //todo implement zobrist hashing and store hashes instead of full board states
    history: Vec<Chessboard>,

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

    pub fn get_castling_en_passant(&self) -> u8 {
        self.castling_en_passant.clone()
    }
    pub fn set_castling_en_passant(&mut self, castling_en_passant: u8) {
        self.castling_en_passant = castling_en_passant;
    }
    pub fn get_history(&self) -> &Vec<Chessboard> {
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
        self.set_castling_en_passant(0b1111_0000); //all castling rights available, no en passant
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
        // Save current state to history (for undo)
        let mut state_to_save = self.clone();
        // Save only the board state, undo then clones the full history back in
        state_to_save.history = Vec::new();
        self.history.push(state_to_save);

        let from = mv.get_from_mask();
        let to = mv.get_to_mask();
        let piece = self.get_piece_at(from);
        let (piece_type, color) = piece;

        //if en passant is assigned, clear it, since we're making a move
        zobrist.toggle_en_passant(&mut self.zobrist_hash, (self.castling_en_passant & 0b0000_1111) as usize);
        self.castling_en_passant &= 0b1111_0000;

        self.assign_en_passant(from, to, piece_type, color, zobrist);

        /*todo castling and en passant
       let old_ep = self.castling_en_passant & 0x0F;
       if old_ep != 0 {
           let old_file = (old_ep as usize) % 8;
           zobrist.toggle_en_passant(&mut self.zobrist_hash, old_file);
       }

       // Handle castling rights changes
       let old_castling = self.castling_en_passant >> 4;
        }*/


       self.move_piece(piece_type, color, from, to, zobrist);

       /*todo  check if castling rights changed (rook/king moved or rook captured)
       let new_castling = self.update_castling_rights(piece_type, color, from, to);

       XOR out old castling rights, XOR in new ones
       for i in 0..4 {
           let old_bit = (old_castling >> i) & 1;
           let new_bit = (new_castling >> i) & 1;
           if old_bit != new_bit {
               zobrist. toggle_castling(&mut self. zobrist_hash, i);
           }
       }*/

       /* todo en passant using files
       let new_ep = self.calculate_en_passant(piece_type, color, from, to);
       if new_ep != 0 {
           let new_file = (new_ep as usize) % 8;
           zobrist.toggle_en_passant(&mut self.zobrist_hash, new_file);
       }*/

       // Toggle side to move
       zobrist.toggle_side(&mut self.zobrist_hash);

   }

    fn assign_en_passant(&mut self, from: u64, to: u64, piece_type: u8, color: i8, zobrist: &ZobristTable) {
        //setting en passant in case the move is a two square pawn push
        if (piece_type, color) == (PAWN, WHITE) && from << 16 == to {
            // transforming the square into a number that fits into 4 bits
            let en_passant_index = ((from << 8).trailing_zeros() - 16) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, en_passant_index as usize);
            //zero en passant bits
            let current_state = self.castling_en_passant & 0b1111_0000;
            //add new
            self.set_castling_en_passant(current_state | en_passant_index)
        } else if (piece_type, color) == (PAWN, BLACK) && from >> 16 == to {
            let en_passant_index = ((from >> 8).trailing_zeros() - 32) as u8;
            zobrist.toggle_en_passant(&mut self.zobrist_hash, en_passant_index as usize);
            //zero en passant bits but keep castling
            let current_state = self.castling_en_passant & 0b1111_0000;
            //add new
            self.set_castling_en_passant(current_state | en_passant_index)
        }
    }

    /**
     Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
     The move_piece and remove_piece functions assume that the move is valid and legal
    */
   //todo en pessant, castling, promotion
   fn move_piece(&mut self, piece: u8, color: i8, from: u64, to: u64, zobrist: &ZobristTable) {
       let array_index= piece_index(piece, color).unwrap();

       let from_sq = from.trailing_zeros() as usize;
       let to_sq = to.trailing_zeros() as usize;

       //XOR out the moving piece from its origin square
       zobrist.toggle_piece(&mut self.zobrist_hash, piece, color, from_sq);

        //XOR out any piece on the destination
        for p in 1..=6 {
            if (self.get_piece_mask(p, WHITE) & to) != 0 {
                zobrist. toggle_piece(&mut self.zobrist_hash, p, WHITE, to_sq);
                self.set_piece_mask(p, WHITE, self.get_piece_mask(p, WHITE) & !to);
                self.white_pieces &= ! to;
            }
            if (self.get_piece_mask(p, BLACK) & to) != 0 {
                zobrist.toggle_piece(&mut self.zobrist_hash, p, BLACK, to_sq);
                self.set_piece_mask(p, BLACK, self.get_piece_mask(p, BLACK) & !to);
                self.black_pieces &= !to;
            }
        }

        // en passant
        // if a pawn is moving diagonally without anything on the destination square, remove a piece behind it
        if (piece, color) == (PAWN, WHITE) && ((from << 7) == to || (from << 9) == to) && ((self.black_pieces & to) == 0) {
            // Clear the captured black pawn from its original square (behind the destination)
            let captured_square = to >> 8; // The square where the captured black pawn was
            self.set_piece_mask(PAWN, BLACK, self.get_piece_mask(PAWN, BLACK) & !captured_square); // Clear from black pawn bitboard
        } else if (piece, color) == (PAWN, BLACK) && ((from >> 7) == to || (from >> 9) == to) && ((self.white_pieces & to) == 0) {
            // Clear the captured white pawn from its original square (behind the destination)
            let captured_square = to << 8; // The square where the captured white pawn was
            self.set_piece_mask(PAWN, WHITE, self.get_piece_mask(PAWN, WHITE) & !captured_square); // Clear from white pawn bitboard
        }

       //XOR in the moving piece at its destination
       zobrist.toggle_piece(&mut self.zobrist_hash, piece, color, to_sq);


       self.pieces[array_index] &= !from; //remove piece from starting square
       self.pieces[array_index] |= to; //add piece to destination square

       if (self.white_pieces & from) != 0 {
           self.white_pieces &= !from;
           self.white_pieces |= to;
       } else if (self.black_pieces & from) != 0 {
           self.black_pieces &= !from;
           self.black_pieces |= to;
       } else {
           panic!("No piece at from square");
       }
   }

   /// Undo the last move by setting the current board state to the previous state
   pub fn undo_move(&mut self) {
       if let Some(mut prev_state) = self.history.pop() {
           // We need to move the current history (minus the popped state) back into the restored state
           prev_state.history = std::mem::take(&mut self.history);
           *self = prev_state;
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
               let square = (file + rank * 8);
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

   fn print_history(&self) {
       println!("Chessboard history \n -----------------");
       let mut i = 0;
       for entry in self.history.clone() {
           println!("Entry {}: ", i);
           entry.print_board();
           i += 1;
       }
   }
}

/// helper: map piece (1..6) and color (WHITE/BLACK) to pieces[] index (0..11)
pub fn piece_index(piece: u8, color: i8) -> Option<usize> {
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
               for square in 0.. 64 {
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
}

impl ZobristTable {
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

       //castling rights (from castling_en_passant byte, upper 4 bits)
       let castling = board.get_castling_en_passant() >> 4;
       for i in 0..4 {
           if (castling >> i) & 1 != 0 {
               hash ^= self.castling_keys[i];
           }
       }

       //en passant file (lower 4 bits encode the file if active)
       let ep_data = board.get_castling_en_passant() & 0x0F;
       if ep_data != 0 {
           // Square from the 0-15 square format
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