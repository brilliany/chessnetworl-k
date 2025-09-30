use config::{FileFormat, Config, File, Source};
use config::ValueKind::I128;
use crate::engine::Engine;

mod chessboard;
mod movegenerator;
mod r#move;
mod engine;
mod heuristics;
#[path = "api/server.rs"] mod server;
mod pieces;


fn main() {
    setup();
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

fn print_bitboard_as_chessboard(board: u64) {
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