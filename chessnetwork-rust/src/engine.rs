use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::DerefMut;

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
use crate::EMPTY;
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::heuristics::Heuristics;
use crate::pieces::Color::White;

const MIN_SCORE: i32 = i32::MIN + 30_000;
const MAX_SCORE: i32 = i32::MAX - 30_000;

//todo separate analysis and search mode, analysis takes a depth parameter, search takes a time parameter and uses iterative deepening
//todo maybe store scoring parameters in a config?
//todo zobrist hashing for transposition table, big speedup

#[derive(Clone)]
enum Mode {
    Analysis,
    Search,
}

//todo maybe add this to config
// 30 seconds time cutoff in ms for search mode
const TIME_CUTOFF: u64 = 30000;

#[derive(Clone)]
pub(crate) struct Engine {
    mode: Mode,
    depth: i32,
    color: i8,
    max_score: i32,
    min_score: i32,
    transposition_table: Arc<Mutex<TranspositionTable>>,
    killer_moves: Arc<Mutex<HashMap<Move, i32>>>,
}

impl Engine {

    pub fn new_single(depth: i32, color: i8) -> Self {
        let transposition_table = Arc::new(Mutex::new(TranspositionTable::new()));
        let killer_moves = Arc::new(Mutex::new(HashMap::new()));
        Engine {
            mode: Mode::Search,
            depth,
            color,
            max_score: i32::MAX,
            min_score: i32::MIN,
            transposition_table,
            killer_moves,
        }
    }

    pub fn get_best_move(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        match &self.mode {
            Mode::Analysis => {
                None
            }
            Mode::Search => Some(self.start_single_search(chessboard)),
        }
    }


    fn start_single_search(&mut self, chessboard: &mut Chessboard) -> Move {
        let start = std::time::Instant::now();

        let moves = generate_moves(chessboard, self.color);
        if moves.is_empty() {
            panic!("No moves available");
        }

        let mut best_move: Option<Move> = None;
        let mut best_score = self.min_score;


        let mut last_time = 0u64;
        // Iterative deepening loop
        for current_depth in 1..=self.depth {
            // time
            let start_time = std::time::Instant::now();

            // Time per depth-unit on average increases exponentially, so we can use this to estimate the time for the next depth and stop if we exceed the time limit
            if current_depth > 1 {
                let estimated_time = last_time * (current_depth as u64) * 2;
                if estimated_time > TIME_CUTOFF {
                    println!("Time cutoff reached, stopping search at depth {}", current_depth - 1);
                    break;
                }
            }

            let mut alpha = self.min_score;
            let beta = self.max_score;

            let mut depth_best_move = None;
            let mut depth_best_score = self.min_score;

            for mv in &moves {
                let mut board_clone = chessboard.clone();
                board_clone.make_move(*mv);

                let search = Search {
                    depth: current_depth,
                    maximizing_color: self.color,
                    max_score: self.max_score,
                    min_score: self.min_score,
                };

                let result = search.alpha_beta(
                    current_depth - 1,
                    alpha,
                    beta,
                    self.color * -1, // opponent color
                    false,          // not root
                    &mut board_clone,
                    true,           // allow null move / quiescence if you support it
                    &mut self.transposition_table.lock().unwrap(),
                    &mut self.killer_moves.lock().unwrap(),
                );

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
                "Depth {} finished: best move {:?}, score {}, elapsed {:?}",
                current_depth,
                best_move,
                best_score,
                start.elapsed()
            );
            last_time = start_time.elapsed().as_millis() as u64;
        }

        best_move.expect("No best move found")
    }
}

pub(crate) struct Search {
    depth: i32,
    maximizing_color: i8,
    max_score: i32,
    min_score: i32,
}


//todo this will be removed when zobrist hashing is implemented
fn board_to_key(board: & Chessboard) -> String {
    let mut key = String::new();
    for i in 0..64 {
        let piece = board.get_piece_at(1u64 << i);
        if piece.0 == EMPTY {
            key.push_str("0");
        } else {
            key.push_str(&piece.0.to_string());
        }
    }
    key
}
/** Key to board function
       * undoes the board to key function
 */
fn key_to_board(key: String) -> Chessboard {
    let mut board = Chessboard::new();
    for (i, c) in key.chars().enumerate() {
        let piece = c.to_digit(10).unwrap() as u8;
        if piece != 0 {
            let bit = 1u64 << i;
            board.set_piece_at(bit, (piece, if piece % 2 == 0 { -1 } else { 1 }));
        }
    }
    board
}

// go to the wikipedia page if you want to understand this
impl Search {
    fn alpha_beta(&self, depth: i32, mut alpha: i32, mut beta: i32, color: i8, maximizing_player: bool, chessboard: &mut Chessboard, use_move_ordering: bool, transposition_table: &mut MutexGuard<TranspositionTable>, killer_moves: &mut MutexGuard<HashMap<Move, i32>>) -> Result {
        let board_key: String = board_to_key(chessboard);
        let mut best_move = None;
        let mut best_score = if maximizing_player { self.min_score } else { self.max_score };

        let mut moves = generate_moves(chessboard, color);

        if use_move_ordering {
            moves.sort_by_key(|mov| {
                if killer_moves.contains_key(mov) {
                    -(killer_moves.get(mov).unwrap())
                } else {
                    0
                }
            });
        }
        if depth == 0 {
            let score = Self::evaluate(chessboard, color);
            return Result::new(score, None);
        }
        if moves.len() == 0 {
            return Result::new(self.min_score, None);
        }

        for mov in moves {
            chessboard.make_move(mov);
            let result = self.alpha_beta(depth - 1, alpha, beta, -color, !maximizing_player, chessboard, true, transposition_table, killer_moves);
            chessboard.undo_move();
            let score = result.score;
            if maximizing_player && score > best_score {
                best_move = Some(mov);
                best_score = score;
                alpha = std::cmp::max(alpha, score);
                if beta <= alpha {
                    transposition_table.insert(Entry::new(board_key.clone(), best_score, depth, best_move, LowerBound));
                    killer_moves.insert(best_move.unwrap(), best_score);
                    break;
                }
            } else if !maximizing_player && score < best_score {
                best_move = Some(mov);
                best_score = score;
                beta = std::cmp::min(beta, score);
                if beta <= alpha {
                    transposition_table.insert(Entry::new(board_key.clone(), best_score, depth, best_move, UpperBound));
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
        transposition_table.insert(Entry::new(board_key.clone(), best_score, depth, best_move, bound_type));
        Result::new(best_score, best_move)
    }

    pub(crate) fn evaluate(position: &Chessboard, color: i8) -> i32 {
        let mut score = 0;
        score += Self::material(position, color);
        score += Self::heuristics(position, color);
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
        if for_color == White {
            score += position.get_white_pawns().count_ones() as i32 * 10;
            score += position.get_white_knights().count_ones() as i32 * 30;
            score += position.get_white_bishops().count_ones() as i32 * 35;
            score += position.get_white_rooks().count_ones() as i32 * 50;
            score += position.get_white_queens().count_ones() as i32 * 90;
            score += position.get_white_kings().count_ones() as i32 * 2000;
            score += position.get_black_pawns().count_ones() as i32 * -10;
            score += position.get_black_knights().count_ones() as i32 * -30;
            score += position.get_black_bishops().count_ones() as i32 * -35;
            score += position.get_black_rooks().count_ones() as i32 * -50;
            score += position.get_black_queens().count_ones() as i32 * -90;
            score += position.get_black_kings().count_ones() as i32 * -2000;
        } else {
            score += position.get_white_pawns().count_ones() as i32 * -10;
            score += position.get_white_knights().count_ones() as i32 * -30;
            score += position.get_white_bishops().count_ones() as i32 * -35;
            score += position.get_white_rooks().count_ones() as i32 * -50;
            score += position.get_white_queens().count_ones() as i32 * -90;
            score += position.get_white_kings().count_ones() as i32 * -2000;
            score += position.get_black_pawns().count_ones() as i32 * 10;
            score += position.get_black_knights().count_ones() as i32 * 30;
            score += position.get_black_bishops().count_ones() as i32 * 35;
            score += position.get_black_rooks().count_ones() as i32 * 50;
            score += position.get_black_queens().count_ones() as i32 * 90;
            score += position.get_black_kings().count_ones() as i32 * 2000;
        }
        score
    }
}
#[derive(Debug, Clone)]
struct TranspositionTable {
    entries: Vec<Entry>,
}

impl TranspositionTable {
    fn new() -> Self {
        TranspositionTable {
            entries: Vec::new(),
        }
    }
    fn get(&self, hash: String) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.hash == hash)
    }
    fn insert(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}
#[derive(Debug, Clone)]
struct Entry {
    hash: String,
    depth: i32,
    score: i32,
    best_move: Option<Move>,
    bound_type: BoundType,
}

impl Entry {
    fn new(hash: String, depth: i32, score: i32, best_move: Option<Move>, bound_type: BoundType) -> Self {
        Entry {
            hash,
            depth,
            score,
            best_move,
            bound_type,
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
