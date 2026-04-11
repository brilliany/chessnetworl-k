use crate::variant::EngineVariant;
use crate::config::TuningConfig;
use chessnetwork_core::{Chessboard, Engine, WHITE, BLACK};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Result of a single game
#[derive(Debug, Clone)]
pub struct GameResult {
    pub white_id: u32,
    pub black_id: u32,
    pub white_score: f32, // 1.0=white win, 0.5=draw, 0.0=black win
}

/// Orchestrates tournament games between variants
pub struct Tournament {
    config: TuningConfig,
}

impl Tournament {
    pub fn new(config: TuningConfig) -> Self {
        Self { config }
    }

    /// Run round-robin tournament (all variants play each other)
    pub fn run_round_robin(&self, population: &mut [EngineVariant]) -> Vec<GameResult> {
        let results = Arc::new(Mutex::new(Vec::new()));

        // Generate all matchups
        let matchups: Vec<_> = (0..population.len())
            .flat_map(|i| {
                (i + 1..population.len())
                    .map(move |j| (i, j))
            })
            .collect();

        if self.config.verbose {
            println!(
                "Running tournament: {} matchups",
                matchups.len()
            );
        }

        // Play games in parallel
        matchups.par_iter().for_each(|(i, j)| {
            // games_per_matchup is total games (evaluating both colors).
            // So we divide by 2 to get the number of pairs. Minimum 1 pair.
            let pairs = (self.config.games_per_matchup.max(2) + 1) / 2;

            for round in 0..pairs {
                // If there are multiple pairs, randomize the first move
                // We pick an opening randomly, but make it fair by using the same opening for both colors.
                let randomize_openings = round > 0;

                let result = self.play_game(
                    &population[*i],
                    &population[*j],
                    true, // white is first variant
                    randomize_openings,
                );
                results.lock().unwrap().push(result);

                // Also play reversed (second variant as white)
                let result_reversed = self.play_game(
                    &population[*j],
                    &population[*i],
                    false, // white is second variant
                    randomize_openings, // Use same randomization strategy
                );
                results.lock().unwrap().push(result_reversed);
            }
        });

        let all_results = results.lock().unwrap().clone();

        // Update population statistics from results
        for result in &all_results {
            if let Some(variant) = population.iter_mut().find(|v| v.id == result.white_id) {
                variant.record_game_result(result.white_score);
            }
            if let Some(variant) = population.iter_mut().find(|v| v.id == result.black_id) {
                variant.record_game_result(1.0 - result.white_score);
            }
        }

        // Calculate fitness for all variants
        for variant in population.iter_mut() {
            variant.calculate_fitness();
        }

        all_results
    }

    /// Play a single game between two variants
    fn play_game(
        &self,
        white_variant: &EngineVariant,
        black_variant: &EngineVariant,
        white_is_first: bool,
        random_first_move: bool,
    ) -> GameResult {
        let mut board = Chessboard::default();
        board.init();

        // Get actual running thread count in rayon, maxed to 1 just in case
        let active_threads = rayon::current_num_threads().max(1);
        let memory_per_engine = (self.config.available_memory / (active_threads * 2)).max(1);

        let mut white_engine = Engine::new_single(self.config.search_depth, chessnetwork_core::WHITE, (&white_variant.params).into(), memory_per_engine);
        let mut black_engine = Engine::new_single(self.config.search_depth, chessnetwork_core::BLACK, (&black_variant.params).into(), memory_per_engine);

        let mut game_ended = false;
        let mut white_won = false;
        let mut black_won = false;

        let output_frequency = 10; // Print board every 10 moves for debugging
        // Play until depth limit or game ends
        for move_count in 0..100 {
            // White move
            let white_move = if move_count == 0 && random_first_move {
                let moves = chessnetwork_core::movegenerator::generate_moves(&mut board, chessnetwork_core::WHITE);
                if !moves.is_empty() {
                    use rand::Rng;
                    Some(moves[rand::thread_rng().gen_range(0..moves.len())])
                } else { None }
            } else {
                white_engine.get_best_move(&mut board)
            };

            if let Some(m) = white_move {
                board.make_move(m);
            } else {
                // White is checkmated or stalemated. Treat as black win for simplification.
                black_won = true;
                game_ended = true;
                break;
            }

            // Black move
            if let Some(black_move) = black_engine.get_best_move(&mut board) {
                board.make_move(black_move);
            } else {
                // Black is checkmated or stalemated. Treat as white win.
                white_won = true;
                game_ended = true;
                break;
            }
            if self.config.verbose && move_count % output_frequency == 0 {
                // use white_is_first to know who is white so it maps correctly in output
                let white_id = if white_is_first { white_variant.id } else { black_variant.id };
                let black_id = if white_is_first { black_variant.id } else { white_variant.id };
                println!("Variant {} (white) vs Variant {} (black) have reached move {}:", white_id, black_id, move_count);
            }
        }

        // Simplified scoring:
        let white_id = if white_is_first { white_variant.id } else { black_variant.id };
        let black_id = if white_is_first { black_variant.id } else { white_variant.id };

        let white_score = if white_won {
            if self.config.verbose {
                println!("Variant {} (white) wins against Variant {} (black)", white_id, black_id);
                board.print_board();
            }
            1.0
        } else if black_won {
            if self.config.verbose {
                println!("Variant {} (black) wins against Variant {} (white)", black_id, white_id);
                board.print_board();
            }
            0.0
        } else {
            if self.config.verbose {
                println!("Variant {} (white) draws against Variant {} (black)", white_id, black_id);
                board.print_board();
            }
            0.5
        };

        GameResult {
            white_id,
            black_id,
            white_score,
        }
    }
}