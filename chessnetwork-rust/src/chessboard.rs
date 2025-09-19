use std::sync::mpsc::channel;
use crate::r#move::Move;
use crate::{BISHOP, BLACK, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE};

#[derive(Default)]
#[derive(Clone)]

pub(crate) struct Chessboard {
    //pieces stored in bitboards
    white_pawns: u64,
    black_pawns: u64,
    white_knights: u64,
    black_knights: u64,
    white_bishops: u64,
    black_bishops: u64,
    white_rooks: u64,
    black_rooks: u64,
    white_queens: u64,
    black_queens: u64,
    white_kings: u64,
    black_kings: u64,
    white_pieces: u64,
    black_pieces: u64,
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

//todo, piece getters and setters dont need to be color specific
impl Chessboard {
    // getters and setters for pieces and castling rights
    pub fn get_white_pawns(&self) -> u64 {
        self.white_pawns
    }
    pub fn set_white_pawns(&mut self, white_pawns: u64) {
        self.white_pawns = white_pawns;
    }
    pub fn get_black_pawns(&self) -> u64 {
        self.black_pawns
    }
    pub fn set_black_pawns(&mut self, black_pawns: u64) {
        self.black_pawns = black_pawns;
    }
    pub fn get_white_knights(&self) -> u64 {
        self.white_knights
    }
    pub fn set_white_knights(&mut self, white_knights: u64) {
        self.white_knights = white_knights;
    }
    pub fn get_black_knights(&self) -> u64 {
        self.black_knights
    }
    pub fn set_black_knights(&mut self, black_knights: u64) {
        self.black_knights = black_knights;
    }
    pub fn get_white_bishops(&self) -> u64 {
        self.white_bishops
    }
    pub fn set_white_bishops(&mut self, white_bishops: u64) {
        self.white_bishops = white_bishops;
    }
    pub fn get_black_bishops(&self) -> u64 {
        self.black_bishops
    }
    pub fn set_black_bishops(&mut self, black_bishops: u64) {
        self.black_bishops = black_bishops;
    }
    pub fn get_white_rooks(&self) -> u64 {
        self.white_rooks
    }
    pub fn set_white_rooks(&mut self, white_rooks: u64) {
        self.white_rooks = white_rooks;
    }
    pub fn get_black_rooks(&self) -> u64 {
        self.black_rooks
    }
    pub fn set_black_rooks(&mut self, black_rooks: u64) {
        self.black_rooks = black_rooks;
    }
    pub fn get_white_queens(&self) -> u64 {
        self.white_queens
    }
    pub fn set_white_queens(&mut self, white_queens: u64) {
        self.white_queens = white_queens;
    }
    pub fn get_black_queens(&self) -> u64 {
        self.black_queens
    }
    pub fn set_black_queens(&mut self, black_queens: u64) {
        self.black_queens = black_queens;
    }
    pub fn get_white_kings(&self) -> u64 {
        self.white_kings
    }
    pub fn set_white_kings(&mut self, white_kings: u64) {
        self.white_kings = white_kings;
    }
    pub fn get_black_kings(&self) -> u64 {
        self.black_kings
    }
    pub fn set_black_kings(&mut self, black_kings: u64) {
        self.black_kings = black_kings;
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
    pub fn set_castling_en_passant(&mut self, castling_rights: u8) {
        self.castling_en_passant = castling_rights;
    }
    pub fn get_history(&self) -> &Vec<Chessboard> {
        &self.history
    }
    pub fn init(&mut self) {
        //set white pieces, structured like this for readability 1 << (square index)
        for i in 0..8 {
            self.set_white_pawns(self.get_white_pawns() | (1 << (48 + i)));
            self.set_black_pawns(self.get_black_pawns() | (1 << (8 + i)));
        }
        self.set_white_knights(self.get_white_knights() | (1 << 57) | (1 << 62));
        self.set_black_knights(self.get_black_knights() | (1 << 1) | (1 << 6));
        self.set_white_bishops(self.get_white_bishops() | (1 << 58) | (1 << 61));
        self.set_black_bishops(self.get_black_bishops() | (1 << 2) | (1 << 5));
        self.set_white_rooks(self.get_white_rooks() | (1 << 56) | (1 << 63));
        self.set_black_rooks(self.get_black_rooks() | (1 << 0) | (1 << 7));
        self.set_white_queens(self.get_white_queens() | (1 << 59));
        self.set_black_queens(self.get_black_queens() | (1 << 3));
        self.set_white_kings(self.get_white_kings() | (1 << 60));
        self.set_black_kings(self.get_black_kings() | (1 << 4));

        self.set_white_pieces(
            self.get_white_pawns()
                | self.get_white_knights()
                | self.get_white_bishops()
                | self.get_white_rooks()
                | self.get_white_queens()
                | self.get_white_kings(),
        );
        self.set_black_pieces(
            self.get_black_pawns()
                | self.get_black_knights()
                | self.get_black_bishops()
                | self.get_black_rooks()
                | self.get_black_queens()
                | self.get_black_kings(),
        );
        self.set_castling_en_passant(1u8 );
    }
    pub fn get_piece_at(&self, pos: u8) -> i8 {
        return if (self.get_white_pawns() & (1 << pos)) != 0 {
            WHITE * PAWN
        } else if (self.get_black_pawns() & (1 << pos)) != 0 {
            BLACK * PAWN
        } else if (self.get_white_knights() & (1 << pos)) != 0 {
            WHITE * KNIGHT
        } else if (self.get_black_knights() & (1 << pos)) != 0 {
            BLACK * KNIGHT
        } else if (self.get_white_bishops() & (1 << pos)) != 0 {
            WHITE * BISHOP
        } else if (self.get_black_bishops() & (1 << pos)) != 0 {
            BLACK * BISHOP
        } else if (self.get_white_rooks() & (1 << pos)) != 0 {
            WHITE * ROOK
        } else if (self.get_black_rooks() & (1 << pos)) != 0 {
            BLACK * ROOK
        } else if (self.get_white_queens() & (1 << pos)) != 0 {
            WHITE * QUEEN
        } else if (self.get_black_queens() & (1 << pos)) != 0 {
            BLACK * QUEEN
        } else if (self.get_white_kings() & (1 << pos)) != 0 {
            WHITE * KING
        } else if (self.get_black_kings() & (1 << pos)) != 0 {
            BLACK * KING
        } else {
            0
        }
    }
    pub(crate) fn print_board(&self) {
        println!("  A B C D E F G H");
        println!("  - - - - - - - -");
        for i in 0..8 {
            print!("{}|", 8 - i);
            for j in 0..8 {
                let piece = self.get_piece_at(i * 8 + j);
                if piece == 0 {
                    print!("  |");
                } else {
                    if piece < 0 {
                        print!("{}|", piece);
                    } else {
                        print!(" {}|", piece);
                    }
                }
            }
            println!();
        }
    }

    // Make a move and add the current state of the board to the history stack
    pub fn make_move(&mut self, mv: Move) {
        // Add the current board state to the history stack
        self.history.push(self.clone());

        // Update the board state
        let from_x = mv.get_from_x();
        let from_y = mv.get_from_y();
        let to_x = mv.get_to_x();
        let to_y = mv.get_to_y();
        let piece = self.get_piece_at(from_x + from_y * 8);
        if piece == 0 {
            self.print_board();
            panic!("No piece at {} {}", from_x, from_y)
        }
        let color = piece / piece.abs();

        self.move_piece(from_x as usize, from_y as usize, to_x as usize, to_y as usize, piece.abs(), color);
    }

    /**
     * Move a piece from one square to another ONLY ON THE PIECE'S OWN BITBOARD
     * @param from_x The x coordinate of the starting square
     * @param from_y The y coordinate of the starting square
     * @param to_x The x coordinate of the destination square
     * @param to_y The y coordinate of the destination square
     * @param piece The piece to move
     * @param color The color of the piece to move
     */
    fn move_piece(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize, piece: i8, color: i8) {
        let shift = (from_x + from_y * 8) % 64;
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
        //update white and black piece bitboards
        self.set_white_pieces(self.get_white_pawns() | self.get_white_knights() | self.get_white_bishops() | self.get_white_rooks() | self.get_white_queens() | self.get_white_kings());
        self.set_black_pieces(self.get_black_pawns() | self.get_black_knights() | self.get_black_bishops() | self.get_black_rooks() | self.get_black_queens() | self.get_black_kings());
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
                if piece == EMPTY {
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