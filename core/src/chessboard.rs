use crate::r#move::{Move, MoveType};
use crate::*;


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
}

impl Chessboard {
    // getters and setters for pieces and castling rights
    pub fn get_piece_mask(&self, piece: u8, color: u8) -> u64 {
        self.pieces[piece_index(piece, color).unwrap()]
    }

    /// set the bitboard for a specific piece type and color NOTE: does not update white_pieces or black_pieces bitboards
    pub fn set_piece_mask(&mut self, piece: u8, color: u8, mask: u64) {
        self.pieces[piece_index(piece, color).unwrap()] = mask;
    }
    
    pub fn get_pieces(&self) -> [u64; 12] {
        self.pieces.clone()
    }
    
    ///overwrite the entire pieces array
    pub fn set_pieces(&mut self, pieces: [u64; 12]) {
        self.pieces = pieces;
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

    /// Create a new chessboard with the standard starting position
    pub(crate) fn new() -> Self {
        let mut board = Chessboard {
                pieces: [0; 12],
                white_pieces: 0,
                black_pieces: 0,
                castling_rights: 0b1111,
                en_passant: 0,
        };
        
        // Apply placements
        for &(piece_type, color, mask) in STARTING_POS {
            board.set_piece_mask(piece_type, color, mask);
            if color == WHITE {
                board.white_pieces |= mask;
            } else {
                board.black_pieces |= mask;
            }
        }
        
        board
    }

    pub fn get_piece_at(&self, pos: u64) -> (u8, u8) {
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
                    if piece.1 == BLACK {
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

    fn assign_en_passant(&mut self, from: u64, piece_type: u8, color: u8) {
        // Set en_passant to the file (1–8) of the pawn that just double-pushed, or 0 for none.
        if (piece_type, color) == (PAWN, WHITE) {
            // from is on rank 1 (bits 8–15); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            self.en_passant = file;
        } else if (piece_type, color) == (PAWN, BLACK) {
            // from is on rank 6 (bits 48–55); file = bit_index % 8, stored 1-based
            let file = (from.trailing_zeros() % 8 + 1) as u8;
            self.en_passant = file;
        }
    }

    pub(crate) fn update_castling_rights(&mut self, from: u64, to: u64) {
        let old_castling = self.castling_rights;
        let mut new_castling = old_castling;

        if (from & (1u64 << 3)) != 0 {
            new_castling &= !WHITE_QUEENSIDE_CASTLE; // White queenside
            new_castling &= !WHITE_KINGSIDE_CASTLE;  // White kingside
        }

        if (from & (1u64 << 59)) != 0 {
            new_castling &= !BLACK_QUEENSIDE_CASTLE; // Black queenside
            new_castling &= !BLACK_KINGSIDE_CASTLE;  // Black kingside
        }

        // Check if rooks are moved or captured (a1, h1, a8, h8)
        if (from & (1u64 << 7)) != 0 || (to & (1u64 << 7)) != 0 {
            new_castling &= !WHITE_QUEENSIDE_CASTLE; // White queenside
        }
        if (from & (1u64 << 0)) != 0 || (to & (1u64 << 0)) != 0 {
            new_castling &= !WHITE_KINGSIDE_CASTLE; // White kingside
        }
        if (from & (1u64 << 63)) != 0 || (to & (1u64 << 63)) != 0 {
            new_castling &= !BLACK_QUEENSIDE_CASTLE; // Black queenside
        }
        if (from & (1u64 << 56)) != 0 || (to & (1u64 << 56)) != 0 {
            new_castling &= !BLACK_KINGSIDE_CASTLE; // Black kingside
        }

        if new_castling != old_castling {
            self.castling_rights = new_castling;
        }
    }

    /**
     Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
     The move_piece and remove_piece functions assume that the move is valid and legal
    */
    pub(crate) fn move_piece(&mut self, piece: u8, color: u8, mv: &Move) { 
        // clear en passant square
        self.en_passant = 0;
        
       let from = mv.get_from_mask();
       let to = mv.get_to_mask();
       let array_index = piece_index(piece, color).unwrap();
       // Remove any piece on the destination square (normal capture)
       self.clear_square(to);

       // Handle special moves
       match mv.move_type() {
           MoveType::Normal => {}
           MoveType::EnPassant { captured_square } => {
               self.clear_square(captured_square);
           }
           MoveType::Castling { rook_from, rook_to } => {
               //update hash table
               let rook_idx = piece_index(ROOK, color).unwrap();
               self.pieces[rook_idx] &= !rook_from;
               self.pieces[rook_idx] |= rook_to;
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
               return; // Promotion is a special case, so we return early after handling it
           },
           MoveType::DoublePawnPush => {
               self.assign_en_passant(from, piece, color);
           }
       }
        

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

   fn clear_square(&mut self, square: u64) {
       let sq = square.trailing_zeros() as usize;
       for p in 1..=6u8 {
           if (self.get_piece_mask(p, WHITE) & square) != 0 {
               self.set_piece_mask(p, WHITE, self.get_piece_mask(p, WHITE) & !square);
               self.white_pieces &= !square;
           }
           if (self.get_piece_mask(p, BLACK) & square) != 0 {
               self.set_piece_mask(p, BLACK, self.get_piece_mask(p, BLACK) & !square);
               self.black_pieces &= !square;
           }
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
               json.push_str(",\n");
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
fn piece_index(piece: u8, color: u8) -> Option<usize> {
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