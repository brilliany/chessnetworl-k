use crate::{Chessboard, Move};
use crate::{WHITE, BLACK};

#[derive(Clone, Debug)]
pub struct Game {
    board: Chessboard,
    turn: u8,           // WHITE or BLACK
    status: GameStatus,
    //history stack for undoing moves
    history: Vec<BoardState>,
}

#[derive(Clone, Debug)]
pub enum GameStatus {
    Ongoing,
    Checkmate(u8),  // losing color
    Stalemate,
    Draw,
}

pub enum IllegalMove {
    InvalidMove,
    NotPlayersTurn,
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
    status: GameStatus,
}

impl Game {
    ///Create a new game with the standard starting position
    pub fn new() -> Self {
        Game {
            board: Chessboard::new(),
            turn: WHITE,
            status: GameStatus::Ongoing,
            history: Vec::new(),
        }
    }
    ///Create a game from a given chessboard and turn e.g. for resuming a game
    pub fn from_chessboard(board: Chessboard, turn: u8) -> Self {
        Game {
            board,
            turn,
            status: GameStatus::Ongoing,
            history: Vec::new(),
        }
    }
    
    pub fn make_move(&mut self, mv: Move) -> Result<GameStatus, IllegalMove> {
        // Save current state to history
        self.history.push(BoardState {
            pieces: self.board.get_pieces(),
            white_pieces: self.board.get_white_pieces(),
            black_pieces: self.board.get_black_pieces(),
            castling_rights: self.board.get_castling_rights(),
            en_passant: self.board.get_en_passant(),
            status: self.status.clone(),
        });
    
        self.board.move_piece(&mv);
    
        // Switch turn
        self.turn = if self.turn == WHITE { BLACK } else { WHITE };
    
        // TODO Check for check, checkmate, stalemate
    
        Ok(self.status.clone())
    }
    
    /// Undo the last move by restoring the previous board state
    pub fn undo_state(&mut self) {
        if let Some(state) = self.history.pop()
        {
            self.board.set_pieces(state.pieces);
            self.board.set_white_pieces(state.white_pieces);
            self.board.set_black_pieces(state.black_pieces);
            self.board.set_castling_rights(state.castling_rights);
            self.board.set_en_passant(state.en_passant);
            // Switch turn back
            self.turn = if self.turn == WHITE { BLACK } else { WHITE };
            self.status = state.status;
        } else {
            println!("No history to undo");
        }
    }
    
    pub fn status(&self) -> GameStatus {
        self.status.clone()
    }
    pub fn board(&self) -> &Chessboard {
        &self.board
    }
    pub fn turn(&self) -> u8 {
        self.turn
    }
    pub fn get_history(&self) -> &Vec<BoardState> {
        &self.history
    }
}