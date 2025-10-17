use std::sync::mpsc::channel;
use crate::*;
use crate::r#move::Move;


//starting position bitboards for each piece type
const STARTING_POS : &[(u8, i8, u64)] = &[
    (PAWN, WHITE, 0x000000000000FF00),
    (PAWN, BLACK, 0x00FF000000000000),
    (KNIGHT, WHITE, (1u64 << 57) | (1u64 << 62)),
    (KNIGHT, BLACK, (1u64 << 1)  | (1u64 << 6)),
    (BISHOP, WHITE, (1u64 << 58) | (1u64 << 61)),
    (BISHOP, BLACK, (1u64 << 2)  | (1u64 << 5)),
    (ROOK, WHITE, (1u64 << 56) | (1u64 << 63)),
    (ROOK, BLACK, (1u64 << 0)  | (1u64 << 7)),
    (QUEEN, WHITE, (1u64 << 59)),
    (QUEEN, BLACK, (1u64 << 3)),
    (KING, WHITE, (1u64 << 60)),
    (KING, BLACK, (1u64 << 4)),
];


#[derive(Default)]
#[derive(Clone)]
pub(crate) struct Chessboard {
    //pieces stored in bitboards
    pieces: [u64; 12], //array of bitboards for each piece type, index 0-5 for white pieces, 6-11 for black pieces
    //order: pawn, knight, bishop, rook, queen, king

    white_pieces: u64, //bitboard for all white pieces

    black_pieces: u64, //bitboard for all black pieces


    //castling rights stored in bit flags, 1 for right, 0 for no right
    //first bit: black kingside, second bit: black queenside, third bit: white kingside, fourth bit: white queenside

    //castling in 4 bits starting from top left (0, 0), just a bit flag for if castling is available for that corner
    //en passant is the remaining 4 bits, read as a 4 bit number 0-15, indexing a square on the 2 middle ranks where en passant is possible
    /* below numbered the squares for en passant
      A B C D E F G H
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    1 2 3 4 5 6 7 8
    9 10 11 12 13 14 15
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
    0 0 0 0 0 0 0 0
     */
    castling_en_passant: u8,
    
    //history stack for undoing moves
    //todo implement zobrist hashing and store hashes instead of full board states
    history: Vec<Chessboard>,
}

impl Chessboard {}


impl Chessboard {
    // getters and setters for pieces and castling rights
    pub fn get_piece_mask(&self, piece: u8, color: i8) -> u64 {
        self.pieces[self.piece_index(piece, color).unwrap()]
    }

    // set the bitboard for a specific piece type and color NOTE: does not update white_pieces or black_pieces bitboards
    pub fn set_piece_mask(&mut self, piece: u8, color: i8, mask: u64) {
        self.pieces[self.piece_index(piece, color).unwrap()] = mask;
    }

    // helper: map piece (1..6) and color (WHITE/BLACK) to pieces[] index (0..11)
    pub fn piece_index(&self, piece: u8, color: i8) -> Option<usize> {
        // a lot of functions depend on this, so panic on invalid input
        if piece == 0 || piece > 6 {
            panic!("Invalid piece type");
        }
        if color == WHITE {
            Some((piece - 1) as usize)
        } else if color == BLACK {
            Some((piece + 5) as usize)
        } else {
            panic!("Invalid color");
        }
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_pieces
    }
    pub fn set_white_pieces(&mut self, white_pieces: u64) {
        self.white_pieces = white_pieces;
    }
    pub fn get_black_pieces(&self) -> u64 {
        self.black_pieces
    }
    pub fn set_black_pieces(&mut self, black_pieces: u64) {
        self.black_pieces = black_pieces;
    }

    pub fn get_castling_en_passant(&self) -> u8 {
        self.castling_en_passant
    }
    pub fn set_castling_en_passant(&mut self, castling_en_passant: u8) {
        self.castling_en_passant = castling_en_passant;
    }
    pub fn get_history(&self) -> &Vec<Chessboard> {
        &self.history
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
        let mask = 1u64 << pos;
        for piece in 1..=6 {
            if (self.get_piece_mask(piece, WHITE) & mask) != 0 {
                return (piece, WHITE);
            }
            if (self.get_piece_mask(piece, BLACK) & mask) != 0 {
                return (piece, BLACK);
            }
        }
        (EMPTY, NONE)
    }

    pub(crate) fn print_board(&self) {
        println!("  A B C D E F G H");
        println!("  - - - - - - - -");
        for i in 0..8 {
            print!("{}|", 8 - i);
            for j in 0..8 {
                let piece = self.get_piece_at(1u64 << (j + (7 - i) * 8));
                let piece_type = piece.0;
                if piece_type == EMPTY {
                    print!("  |");
                } else {
                    if (piece_type as usize) < 0 {
                        print!("{:?}|", piece_type);
                    } else {
                        print!(" {:?}|", piece_type);
                    }
                }
            }
            println!();
        }
    }

    // Make a move and add the current state of the board to the history stack
    pub fn make_move(&mut self, mv: Move) {
        //todo refactor move first

        // Add the current board state to the history stack
        self.history.push(self.clone());

        // Update the board state
        /*let from_x = mv.get_from_x();
        let from_y = mv.get_from_y();
        let to_x = mv.get_to_x();
        let to_y = mv.get_to_y();
        let piece = self.get_piece_at(from_x + from_y * 8);
        let piece_type = piece.get_piece_type();
        let color = piece.get_color();
        if piece_type == EMPTY {
            self.print_board();
            panic!("No piece at {} {}", from_x, from_y)
        };*/

        let from = mv.get_from_mask();
        let to = mv.get_to_mask();
        let (piece_type, color) = self.get_piece_at(from);


        self.move_piece(piece_type, color, from, to);
    }

    /*
      Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
      The move_piece and remove_piece functions should assume that the move is valid and legal similarly to piece_index
     */
    //todo en pessant, castling, promotion
    fn move_piece(&mut self, piece: u8, color: i8, from: u64, to: u64) {
        let array_index = self.piece_index(piece, color).unwrap();

        //remove any piece that might be on the destination square
        for p in 1..=6 {
            if (self.get_piece_mask(p, WHITE) & to) != 0 {
                self.set_piece_mask(p, WHITE, self.get_piece_mask(p, WHITE) & !to);
            }
            if (self.get_piece_mask(p, BLACK) & to) != 0 {
                self.set_piece_mask(p, BLACK, self.get_piece_mask(p, BLACK) & !to);
            }
        }

        self.pieces[array_index] &= !from; //remove piece from starting square
        self.pieces[array_index] |= to; //add piece to destination square

        if self.white_pieces & from != 0 {
            self.white_pieces &= !from;
            self.white_pieces |= to;
        } else if self.black_pieces & from != 0 {
            self.black_pieces &= !from;
            self.black_pieces |= to;
        } else {
            panic!("No piece at from square");
        }
    }

    // Undo the last move by setting the current board state to the previous state
    pub fn undo_move(&mut self) {
        if let Some(prev_state) = self.history.pop() {
            *self = prev_state;
        }
    }

    /**
    * Returns the current board state as a JSON string
    * The JSON string is formatted as follows:
    * {
    *   board: [
    *       [black_rook, black_knight, black_bishop, black_queen, black_king, black_bishop, black_knight, black_rook],
    *       [black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn, black_pawn],
    *       [empty, empty, empty, empty, empty, empty, empty, empty],
        */
    pub(crate) fn convert_to_json(&mut self) -> String {
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("\t\"board\": [\n");
        for rank in (0..8).rev() {
            json.push_str("\t\t[");
            for file in 0..8 {
                let square = rank * 8 + file;
                let (piece, color) = self.get_piece_at(square as u64);
                let string = if color == WHITE { "white" } else if color == BLACK { "black" } else { "empty" };
                let piece_value = match (piece) {
                    PAWN => format!("\"{}_pawn\"", string),
                    KNIGHT => format!("\"{}_knight\"", string),
                    BISHOP => format!("\"{}_bishop\"", string),
                    ROOK => format!("\"{}_rook\"", string),
                    QUEEN => format!("\"{}_queen\"", string),
                    KING => format!("\"{}_king\"", string),
                    EMPTY => "\"empty\"".to_string(),
                    _ => "\"invalid\"".to_string(),
                };
                json.push_str(&format!("{}", piece_value));
                if file < 7 {
                    json.push_str(", ");
                }
            }
            json.push_str("]");
            if rank > 0 {
                json.push_str(",\n");
            } else {
                json.push_str("\n");
            }
        }
        json
    }
}