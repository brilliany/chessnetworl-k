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
    fn piece_index(&self, piece: u8, color: i8) -> Option<usize> {
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



        self.move_piece();
    }

    /*
      Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
      The move_piece and remove_piece functions should assume that the move is valid and legal similarly to piece_index
     */
    fn move_piece(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize, piece: i8, color: i8) {
        /*let shift = (from_x + from_y * 8) % 64;
        let mask = 1u64 << shift;
        match (color, piece) {
            (WHITE, PAWN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_pawns((self.get_white_pawns() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, PAWN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_pawns((self.get_black_pawns() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (WHITE, KNIGHT) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_knights((self.get_white_knights() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, KNIGHT) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_knights((self.get_black_knights() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (WHITE, BISHOP) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_bishops((self.get_white_bishops() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, BISHOP) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_bishops((self.get_black_bishops() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (WHITE, ROOK) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_rooks((self.get_white_rooks() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, ROOK) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_rooks((self.get_black_rooks() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (WHITE, QUEEN) => {
                // Remove the piece from the starting squareand add it to the destination square
                self.set_white_queens((self.get_white_queens() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, QUEEN) => {
// Remove the piece from the starting square and add it to the destination square
                self.set_black_queens((self.get_black_queens() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (WHITE, KING) => {
// Remove the piece from the starting square and add it to the destination square
                self.set_white_kings((self.get_white_kings() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            (BLACK, KING) => {
// Remove the piece from the starting square and add it to the destination square
                self.set_black_kings((self.get_black_kings() & !mask) | (1 << ((to_x + to_y * 8) % 64)));
            },
            _ => {
                println!("Invalid move");
            }
        }
        */

        self.remove_piece(from_x as u8, from_y as u8, piece, color);
        //update white and black piece bitboards
        self.set_white_pieces(self.get_white_pawns() | self.get_white_knights() | self.get_white_bishops() | self.get_white_rooks() | self.get_white_queens() | self.get_white_kings());
        self.set_black_pieces(self.get_black_pawns() | self.get_black_knights() | self.get_black_bishops() | self.get_black_rooks() | self.get_black_queens() | self.get_black_kings());

        //remove any piece that might be on the destination square

    }

    // Undo the last move by setting the current board state to the previous state
    pub fn undo_move(&mut self) {
        if let Some(prev_state) = self.history.pop() {
            *self = prev_state;
        }
    }

    fn remove_piece(&mut self, x: u8, y: u8, piece: i8, color: i8) {
        match (color, piece) {
            (WHITE, PAWN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_pawns(self.get_white_pawns() & !(1 << (x + y * 8)));
            },
            (BLACK, PAWN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_pawns(self.get_black_pawns() & !(1 << (x + y * 8)));
            },
            (WHITE, KNIGHT) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_knights(self.get_white_knights() & !(1 << (x + y * 8)));
            },
            (BLACK, KNIGHT) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_knights(self.get_black_knights() & !(1 << (x + y * 8)));
            },
            (WHITE, BISHOP) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_bishops(self.get_white_bishops() & !(1 << (x + y * 8)));
            },
            (BLACK, BISHOP) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_bishops(self.get_black_bishops() & !(1 << (x + y * 8)));
            },
            (WHITE, ROOK) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_rooks(self.get_white_rooks() & !(1 << (x + y * 8)));
            },
            (BLACK, ROOK) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_rooks(self.get_black_rooks() & !(1 << (x + y * 8)));
            },
            (WHITE, QUEEN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_queens(self.get_white_queens() & !(1 << (x + y * 8)));
            },
            (BLACK, QUEEN) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_queens(self.get_black_queens() & !(1 << (x + y * 8)));
            },
            (WHITE, KING) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_white_kings(self.get_white_kings() & !(1 << (x + y * 8)));
            },
            (BLACK, KING) => {
                // Remove the piece from the starting square and add it to the destination square
                self.set_black_kings(self.get_black_kings() & !(1 << (x + y * 8)));
            },
            _ => {
                println!("Invalid piece");
            }
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

        for y in 0..8 {
            json.push_str("\t\t[");
            for x in 0..8 {
                let mut piece = self.get_piece_at(x+y*8);
                if piece == None {
                    json.push_str("\"empty\"");
                } else {
                    if piece < 0 {
                        json.push_str("\"black_");
                    } else {
                        json.push_str("\"white_");
                    }
                    match piece.abs() {
                        PAWN => {
                            json.push_str("pawn\"");
                        },
                        KNIGHT => {
                            json.push_str("knight\"");
                        },
                        BISHOP => {
                            json.push_str("bishop\"");
                        },
                        ROOK => {
                            json.push_str("rook\"");
                        },
                        QUEEN => {
                            json.push_str("queen\"");
                        },
                        KING => {
                            json.push_str("king\"");
                        },
                        _ => {
                            println!("Invalid piece");
                        }
                    }
                }
                if x < 7 {
                    json.push_str(", ");
                }
            }
            json.push_str("]");
            if y < 7 {
                json.push_str(",\n");
            }
        }
        json.push_str("\n\t]\n");
        json.push_str("}");
        json
    }

    pub(crate) fn set_piece(&mut self, square: u8, to: i8) {
        match (to, to > 0) {
            (PAWN, true) => {
                self.set_white_pawns(self.get_white_pawns() | (1 << square));
            },
            (PAWN, false) => {
                self.set_black_pawns(self.get_black_pawns() | (1 << square));
            },
            (KNIGHT, true) => {
                self.set_white_knights(self.get_white_knights() | (1 << square));
            },
            (KNIGHT, false) => {
                self.set_black_knights(self.get_black_knights() | (1 << square));
            },
            (BISHOP, true) => {
                self.set_white_bishops(self.get_white_bishops() | (1 << square));
            },
            (BISHOP, false) => {
                self.set_black_bishops(self.get_black_bishops() | (1 << square));
            },
            (ROOK, true) => {
                self.set_white_rooks(self.get_white_rooks() | (1 << square));
            },
            (ROOK, false) => {
                self.set_black_rooks(self.get_black_rooks() | (1 << square));
            },
            (QUEEN, true) => {
                self.set_white_queens(self.get_white_queens() | (1 << square));
            },
            (QUEEN, false) => {
                self.set_black_queens(self.get_black_queens() | (1 << square));
            },
            (KING, true) => {
                self.set_white_kings(self.get_white_kings() | (1 << square));
            },
            (KING, false) => {
                self.set_black_kings(self.get_black_kings() | (1 << square));
            },
            _ => {
                println!("Invalid piece");
            }
        }
    }
}