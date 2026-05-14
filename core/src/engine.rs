use crate::chessboard::Chessboard;
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::heuristics::{HeuristicParams, Heuristics};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::zobrist::ZOBRIST;
use crate::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK};
use crate::{BLACK, WHITE};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;

const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;


const TIME_CUTOFF: u64 = 20000;

pub struct Engine {
    depth: u32,
    color: u8,
    heuristics: HeuristicParams,
    available_memory_mb: usize,
}

/**The main chess engine struct
    * has direct board access for fast move generation and evaluation
**/
impl Engine {
    pub fn new(depth: u32, color: u8, heuristics: HeuristicParams, available_memory_mb: usize) -> Self {

        Engine {
            depth,
            color,
            heuristics,
            available_memory_mb,
        }
    }

    pub fn get_best_move(&mut self, chessboard: Chessboard) -> Option<Move> {
        self.start_single_search(chessboard)
    }

    fn start_single_search(&mut self, chessboard: Chessboard) -> Option<Move> {
        let start = std::time::Instant::now();

        let moves = generate_moves(&chessboard, self.color);
        // println!("{} moves available for engine", moves.len());
        if moves.is_empty() {
            return None;
        }
        
        let transposition_table = Rc::new(TranspositionTable::new(self.available_memory_mb));
        let killer_moves = Rc::new(RefCell::new(HashMap::new()));

        let mut best_score = MIN_SCORE;

        let best_move = moves.iter().map(|mov| {
            let mut chessboard = chessboard.clone();
            chessboard.move_piece(mov);

            let search = Search {
                transposition_table: Rc::clone(&transposition_table),
                killer_moves: Rc::clone(&killer_moves),
                heuristics_params: self.heuristics.clone(),
                start_time: start,
            };
            let result = search.alpha_beta(chessboard, MIN_SCORE, MAX_SCORE, true, self.color ^ 1, self.depth - 1);
            best_score = std::cmp::max(best_score, result.score);
            (*mov, result.score)
        }).max_by_key(|&(_, score)| score).map(|(mov, _)| mov);

        /*for mov in &moves {
            if start.elapsed().as_millis() as u64 > TIME_CUTOFF {
                println!("Time cutoff reached");
                return None;
            }
            chessboard.move_piece(mov);

            let search = Search {
                transposition_table,
                killer_moves: RefCell::new(killer_moves),
                heuristics_params: self.heuristics.clone(),
                start_time: start,
            };
            let result = search.alpha_beta(chessboard, MIN_SCORE, MAX_SCORE, true, self.color, self.depth);

            if result.best_move.is_some() && result.score > best_score {
                best_score = result.score;
                best_move = result.best_move;
            }
        }*/
        

        let time = start.elapsed().as_millis() as u64;
            
        println!("Engine chose move {:?} with score {} in {} ms", best_move, best_score, time);
        best_move
    }
}

struct Search {
    transposition_table: Rc<TranspositionTable>,
    killer_moves: Rc<RefCell<HashMap<Move, i32>>>,
    heuristics_params: HeuristicParams,
    start_time: std::time::Instant,
}

impl Search {
    // go to the wikipedia page if you want to understand this
    fn alpha_beta(&self, mut chessboard: Chessboard, mut alpha: i32, mut beta: i32, maximizing_player: bool, color_to_move: u8, depth: u32,) -> Result {
        if depth == 0 {
            let score = evaluate(&chessboard, color_to_move, &self.heuristics_params);
            return Result::new(score, None);
        }
        let board_key: u64 = ZOBRIST.hash(&chessboard, color_to_move);
        // Check transposition table
        if let Some(entry) = self.transposition_table.get(board_key) {
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
        let mut moves = generate_moves(&chessboard, color_to_move);
        if moves.is_empty() {
            return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
        }
        // Order moves based on scores (e.g., killer moves, history heuristics)
        moves.sort_by_cached_key(|mv| {
            if let Some(score) = self.killer_moves.borrow().get(mv) {
                -*score // Prefer killer moves (higher scores first)
            } else {
                0 // Default score for icons moves
            }
        });
        for mov in moves {
            // Enforce time abort during deep branches
            if self.start_time.elapsed().as_millis() as u64 > TIME_CUTOFF {
                println!("Time cutoff reached, aborting search at depth {}", depth);
                return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
            }

            let saved_state = chessboard.clone(); // Save state for undo
            chessboard.move_piece(&mov);
            let result = self.alpha_beta(chessboard, alpha, beta, !maximizing_player, color_to_move ^ 1, depth - 1);
            chessboard = saved_state; // Undo move
            let score = result.score;
            if maximizing_player && score > best_score {
                best_move = Some(mov);
                best_score = score;
                alpha = std::cmp::max(alpha, score);
                if beta <= alpha {
                    self.transposition_table.insert(Entry {
                        hash: board_key,
                        score: best_score,
                        depth: depth,
                        best_move,
                        bound_type: LowerBound
                    });
                    //if we caused a cutoff, add this move to killer moves so that we skip the branch as soon as possible if we find it again
                    self.killer_moves.borrow_mut().insert(best_move.unwrap(), best_score);
                    break;
                }
            } else if !maximizing_player && score < best_score {
                best_move = Some(mov);
                best_score = score;
                beta = std::cmp::min(beta, score);
                if beta <= alpha {
                    self.transposition_table.insert(Entry {
                        hash: board_key,
                        score: best_score,
                        depth,
                        best_move,
                        bound_type: UpperBound
                    });
                    self.killer_moves.borrow_mut().insert(best_move.unwrap(), best_score);
                    break;
                }
            }
        }
        let bound_type = if best_score <= alpha {
            UpperBound
        } else if best_score >= beta {
            LowerBound
        } else {
            BoundType::Exact
        };
        self.transposition_table.insert(Entry {
            hash: board_key,
            score: best_score,
            depth,
            best_move,
            bound_type});
        Result::new(best_score, best_move)
    }
}
pub fn evaluate(position: &Chessboard, color: u8, params: &HeuristicParams) -> i32 {
    let mut score = 0;
    score += material(position, color);
    score += evaluate_heuristics(position, color, params);
    score
}
pub fn evaluate_heuristics(position: &Chessboard, color: u8, h: &HeuristicParams) -> i32 {
    let mut score: i32 = 0;
    let heuristics_eval = Heuristics::new(position, color);
    score += heuristics_eval.two_middle_pawns() * h.two_middle_pawns_weight;
    score += heuristics_eval.castling() * h.castling_weight;
    score += heuristics_eval.knight_outpost() * h.knight_outpost_weight;
    score += heuristics_eval.development() * h.development_weight;
    score += heuristics_eval.mobility() * h.mobility_weight;
    score
}
pub fn material(position: &Chessboard, for_color: u8) -> i32 {
    let mut score = 0;
    for piece in 1..=6 {
        let piece_value = match piece {
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
            score += position.get_piece_mask(piece, WHITE).count_ones() as i32 * piece_value;
            score -= position.get_piece_mask(piece, BLACK).count_ones() as i32 * piece_value;
        } else {
            score -= position.get_piece_mask(piece, WHITE).count_ones() as i32 * piece_value;
            score += position.get_piece_mask(piece, BLACK).count_ones() as i32 * piece_value;
        }
    }
    score
}

#[derive(Debug, Clone)]
struct TranspositionTable {
    entries: RefCell<Vec<Option<Entry>>>,
    size: usize,
}

#[derive(Debug, Clone)]
struct Entry {
    hash: u64,
    depth: u32,
    score: i32,
    best_move: Option<Move>,
    bound_type: BoundType,
}

impl TranspositionTable {
    fn new(size_mb: usize) -> Self {
        let entry_size = size_of::<Option<Entry>>();
        let num_entries = ((size_mb * 1024 * 1024) / entry_size).max(1);
        TranspositionTable {
            entries: RefCell::new(vec![None; num_entries]),
            size: num_entries,
        }
    }

    fn get(&self, hash: u64) -> Option<Entry> {
        let idx = (hash as usize) % self.size;
        self.entries.borrow()[idx].as_ref().filter(|e| e.hash == hash).cloned()
    }

    fn insert(&self, entry: Entry) {
        let idx = (entry.hash as usize) % self.size;
        let mut entries = self.entries.borrow_mut();
        // Replace the entry if the current depth is greater, or insert it if None
        match entries[idx].as_ref() {
            Some(existing_entry) if entry.depth > existing_entry.depth => {
                entries[idx] = Some(entry);
            }
            None => {
                entries[idx] = Some(entry);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
enum BoundType {
    Exact,
    LowerBound,
    UpperBound,
}
//TODO rename to SearchResult to not shadow std result
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

