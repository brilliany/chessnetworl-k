use chessnetwork_core::{Chessboard, Game, Move};

/**
* Session struct
   * id: String (current time) - identifier for the session, game can be saved with this id
   * board: Chessboard - responsible for storing the state of the board
   * turn: u8 - 1 for white, 0 for black
   * opponent: String - either "human" or "engine"
*/
#[derive(Clone)]
pub(crate) struct Session {
    pub(crate) id: String,
    game: Game,
    user_color: u8,
    opponent_color: u8,
}

impl Session {
    pub(crate) fn new(id: String, game: Game, user_color: u8, opponent_color: u8) -> Session {
        Session {
            id,
            game,
            user_color,
            opponent_color,
        }
    }
    pub(crate) fn get_id(&self) -> String {
        self.id.clone()
    }
    pub(crate) fn get_board_state(&self) -> Chessboard {
        self.game.board().clone()
    }
    pub(crate) fn get_turn(&self) -> u8 {
        self.game.turn()
    }
    pub(crate) fn get_opponent_color(&self) -> u8 {
        self.opponent_color
    }
    pub(crate) fn get_user_color(&self) -> u8 {
        self.user_color
    }
    pub(crate) fn make_move(&mut self, mv: Move) -> bool {
        self.game.make_move(mv).is_ok()
    }
}