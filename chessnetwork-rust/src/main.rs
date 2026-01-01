use config::{FileFormat, Config, File, Source};
use config::ValueKind::I128;
use crate::chessboard::Chessboard;
use crate::engine::{board_to_key, key_to_board, Engine};
use crate::r#move::Move;

mod chessboard;
mod movegenerator;
mod r#move;
mod engine;
mod heuristics;
#[path = "api/server.rs"] mod server;
mod bitboards;
define_bitboard_consts!();

// Piece constants, used instead of enums for performance reasons, also easier to eg. convert a white pawn to a black pawn by multiplying by -1
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;
pub const EMPTY: u8 = 0;

pub const WHITE: i8 = 1;
pub const BLACK: i8 = -1;
pub const NONE: i8 = 0;



fn main() {
    let mut board = Chessboard::default();
    board.init();
    let board_string = board_to_key(&board);
    board.print_board();
    key_to_board(board_string).print_board()
    /*setup();*/
}


fn setup() {
    let config = get_config();
    let result = server::main(config);
    let result_string = match result {
        Ok(_) => "Ok",
        Err(_) => "Err",
    };
    println!("Result: {}", result_string);
}

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

fn get_config() -> Config {
    let config = Config::builder().add_source(File::new("config.yml", FileFormat::Yaml));
    let config1 = config.build().unwrap();
    config1
}

fn get_config_value(config: &Config, key: &str) -> String {
    let mode = config.get::<String>(key).unwrap();
    mode
}
