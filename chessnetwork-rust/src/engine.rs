use rayon::prelude::*;
use std::collections::HashMap;
use crate::{PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING};

/**
    Engine will have two modes:
    1. Continuous search
        - Engine will be running in a loop, using iterative deepening to search for the best move
        - No depth limit, positions will be received from the controller
    2. Single search
        - Engine will be asked to calculate a single move, then exit
        - Depth limit, position will be received from the controller
*/

use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use crate::chessboard::Chessboard;
use crate::{BLACK, EMPTY, WHITE};
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::heuristics::Heuristics;

const MIN_SCORE: i32 = i32::MIN;
const MAX_SCORE: i32 = i32::MAX;

//todo maybe store scoring parameters in a config?

//todo maybe add this to config
// 30 seconds time cutoff in ms for search mode
const TIME_CUTOFF: u64 = 30000;

#[derive(Clone)]
pub(crate) struct Engine {
    depth: i32,
    color: i8,
    transposition_table: TranspositionTable,
    killer_moves: HashMap<Move, i32>,
}

impl Engine {
    pub fn new_single(depth: i32, color: i8) -> Self {
        let transposition_table = TranspositionTable::new(20000);
        let killer_moves = HashMap::new();
        Engine {
            depth,
            color,
            transposition_table,
            killer_moves,
        }
    }

    pub fn get_best_move(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        Some(self.start_single_search(chessboard))
    }


    fn start_single_search(&mut self, chessboard: &mut Chessboard) -> Move {
        let start = std::time::Instant::now();

        let mut moves = generate_moves(chessboard, self.color);
        println!("{} moves available for engine", moves.len());
        if moves.is_empty() {
            panic!("No moves available");
        }

        let mut best_move: Option<Move> = None;
        let mut best_score = MIN_SCORE;


        let mut last_time = 0u64;
        // Iterative deepening loop
        for current_depth in 1..=self.depth {
            //time
            let start_time = std::time::Instant::now();

            //move ordering
            moves.sort_by_cached_key(|m| {
                let killer_score = self.killer_moves.get(m).unwrap_or(&0);
                let transposition_score = if let Some(entry) = self.transposition_table.get(chessboard.get_hash()) {
                    if let Some(tt_move) = entry.best_move {
                        if *m == tt_move { 1000 } else { 0 }
                    } else {
                        0
                    }
                } else {
                    0
                };
                -(killer_score + transposition_score)
            });

            // Time per depth-unit on average increases almost exponentially, so we can use this to estimate the time for the next depth and stop if we exceed the time limit
            if current_depth > 1 {
                let estimated_time = last_time * (current_depth as u64) * 2;
                if estimated_time > TIME_CUTOFF {
                    println!("Time cutoff reached, stopping search at depth {}", current_depth - 1);
                    break;
                }
            }

            let mut alpha = MIN_SCORE;
            let beta = MAX_SCORE;

            let mut depth_best_move = None;
            let mut depth_best_score = MIN_SCORE;

            for mv in &moves {
                chessboard.make_move(*mv);
                let result = alpha_beta(
                    current_depth - 1,
                    alpha,
                    beta,
                    -self.color,
                    false,
                    chessboard,
                    &mut self.transposition_table,
                    &mut self.killer_moves,
                );
                chessboard.undo_move();
                if result.score > depth_best_score {
                    depth_best_score = result.score;
                    depth_best_move = Some(*mv);
                }

                // Fail-hard beta cutoff
                if depth_best_score >= beta {
                    break;
                }

                // Improve alpha
                if depth_best_score > alpha {
                    alpha = depth_best_score;
                }
            }

            if let Some(m) = depth_best_move {
                best_move = Some(m);
                best_score = depth_best_score;
            }

            println!(
                "Depth {} finished: best move {} to {}, score {}, elapsed {:?}, transposition table size {}MB",
                current_depth,
                best_move.unwrap().get_from_mask().trailing_zeros(),
                best_move.unwrap().get_to_mask().trailing_zeros(),
                best_score,
                start.elapsed(),
                std::mem::size_of::<Option<Entry>>()*self.transposition_table.size / (1024*1024)
            );
            last_time = start_time.elapsed().as_millis() as u64;
        }

        best_move.expect("No best move found")
    }
}




// go to the wikipedia page if you want to understand this
fn alpha_beta(depth: i32, mut alpha: i32, mut beta: i32, color: i8, maximizing_player: bool, chessboard: &mut Chessboard, transposition_table: &mut TranspositionTable, killer_moves: &mut HashMap<Move, i32>) -> Result {
    if depth == 0 {
        let eval_color = if maximizing_player { color } else { -color };
        let score = evaluate(chessboard, eval_color);
        /*println!("Reached end of depth");
           chessboard.print_board();*/
        return Result::new(score, None);
    }
    let board_key: u64 = chessboard.get_hash();
    // Check transposition table
    if let Some(entry) = transposition_table.get(board_key) {
        if entry.depth >= depth {
            // Return the stored result if the depth matches or is greater
            match entry.bound_type {
                BoundType::Exact => return Result::new(entry.score, entry.best_move),
                LowerBound if entry.score > alpha => alpha = entry.score,
                UpperBound if entry.score < beta => beta = entry.score,
                _ => {}
            }
            if alpha >= beta {
                return Result::new(entry.score, entry.best_move);
            }
        }
    }
    let mut best_score = if maximizing_player { MIN_SCORE } else { MAX_SCORE };
    let mut best_move = None;
    let mut moves = generate_moves(chessboard, color);
    if moves.len() == 0 {
        return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
    }
    // Order moves based on scores (e.g., killer moves, history heuristics)
    moves.sort_by_cached_key(|mv| {
        if let Some(score) = killer_moves.get(mv) {
            -*score // Prefer killer moves (higher scores first)
        } else {
            0 // Default score for other moves
        }
    });
    for mov in moves {
        chessboard.make_move(mov);
        let result = alpha_beta(depth - 1, alpha, beta, -color, !maximizing_player, chessboard, transposition_table, killer_moves);
        chessboard.undo_move();
        let score = result.score;
        if maximizing_player && score > best_score {
            best_move = Some(mov);
            best_score = score;
            alpha = std::cmp::max(alpha, score);
            if beta <= alpha {
                transposition_table.insert(Entry {
                    hash: board_key,
                    score: best_score,
                    depth,
                    best_move,
                    bound_type: LowerBound});
                killer_moves.insert(best_move.unwrap(), best_score);
                break;
            }
        } else if !maximizing_player && score < best_score {
            best_move = Some(mov);
            best_score = score;
            beta = std::cmp::min(beta, score);
            if beta <= alpha {
                transposition_table.insert(Entry {
                    hash: board_key,
                    score: best_score,
                    depth,
                    best_move,
                    bound_type: UpperBound});
                killer_moves.insert(best_move.unwrap(), best_score);
                break;
            }
        }
    }
    let bound_type;
    if best_score <= alpha {
        bound_type = UpperBound;
    } else {
        bound_type = LowerBound;
    }
    transposition_table.insert(Entry {
        hash: board_key,
        score: best_score,
        depth,
        best_move,
        bound_type});
    Result::new(best_score, best_move)
}
pub(crate) fn evaluate(position: &Chessboard, color: i8) -> i32 {
    let mut score = 0;
    score += material(position, color);
    score += heuristics(position, color);
    score
}
fn heuristics(position: &Chessboard, color: i8) -> i32 {
    let mut score: i32 = 0;
    let heuristics = Heuristics::new(position, color);
    score += heuristics.two_middle_pawns();
    score += heuristics.castling();
    score += heuristics.knight_outpost();
    score += heuristics.development();
    score += heuristics.mobility();
    score
}
fn material(position: &Chessboard, for_color: i8) -> i32 {
    let mut score = 0;
    for i in 1..6 {
        let piece_value = match i {
            PAWN => 10,
            KNIGHT => 30,
            BISHOP => 35,
            ROOK => 50,
            QUEEN => 90,
            //todo implement checkmate detection instead of just fooling the engine
            KING => 9999,
            _ => 0,
        };
        if for_color == WHITE {
            score += position.get_piece_mask((i + 1) as u8, WHITE).count_ones() as i32 * piece_value;
            score -= position.get_piece_mask((i + 1) as u8, BLACK).count_ones() as i32 * piece_value;
        } else {
            score -= position.get_piece_mask((i + 1) as u8, WHITE).count_ones() as i32 * piece_value;
            score += position.get_piece_mask((i + 1) as u8, BLACK).count_ones() as i32 * piece_value;
        }
    }
    score
}

#[derive(Debug, Clone)]
struct TranspositionTable {
    // Use a fixed-size array with index = hash % size for O(1) access
    entries: Vec<Option<Entry>>,
    size: usize,
}

#[derive(Debug, Clone)]
struct Entry {
    hash: u64,
    depth: i32,
    score: i32,
    best_move: Option<Move>,
    bound_type: BoundType,
}

impl TranspositionTable {
    fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<Option<Entry>>();
        let num_entries = (size_mb * 1024 * 1024) / entry_size;
        TranspositionTable {
            entries:  vec![None; num_entries],
            size: num_entries,
        }
    }

    fn get(&self, hash: u64) -> Option<&Entry> {
        let idx = (hash as usize) % self.size;
        self.entries[idx]. as_ref().filter(|e| e.hash == hash)
    }

    fn insert(&mut self, entry: Entry) {
        let idx = (entry.hash as usize) % self.size;
        // Replace the entry if the current depth is greater, or insert it if None
        match self.entries[idx].as_ref() {
            Some(existing_entry) if entry.depth > existing_entry.depth => {
                self.entries[idx] = Some(entry);
            }
            None => {
                self.entries[idx] = Some(entry);
            }
            _ => {} // scrap if theyre the same
        }
    }
}

#[derive(Debug, Clone)]
enum BoundType {
    Exact,
    LowerBound,
    UpperBound,
}
#[derive(Clone)]
struct Result {
    score: i32,
    best_move: Option<Move>,
}

impl Result {
    fn new(score: i32, best_move: Option<Move>) -> Self {
        Result {
            score,
            best_move,
        }
    }
}
