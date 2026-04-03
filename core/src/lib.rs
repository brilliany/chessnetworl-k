pub mod chessboard;
pub mod movegenerator;
pub mod engine;

// Internal
pub mod heuristics;
pub mod config_loader;

mod r#move;
pub mod constants;

define_consts!();

pub use r#move::Move;

pub use chessboard::Chessboard;
pub use engine::Engine;
pub use heuristics::HeuristicParams;
pub use config_loader::load_heuristics_from_config;
pub use config_loader::load_available_memory_from_config;
pub use config_loader::load_benchmarking_from_config;

/// Utility
pub fn print_bitboard(board: u64) {
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