use crate::{BISHOP, KING, KNIGHT, PAWN, QUEEN, ROOK};
use std::collections::HashMap;

use crate::chessboard::Chessboard;
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::heuristics::{HeuristicParams, Heuristics};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::{BLACK, WHITE};
use crate::zobrist::ZobristTable;

const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;


const TIME_CUTOFF: u64 = 2000;

pub struct Engine {
    depth: u32,
    color: u8,
    transposition_table: TranspositionTable,
    killer_moves: HashMap<Move, i32>,
    heuristics: HeuristicParams,
    zobrist_table: ZobristTable
}

/**The main chess engine struct
    * has direct board access for fast move generation and evaluation
**/
impl Engine {
    /// Create a new engine with the given chessboard, search depth, color to play, heuristic parameters, and available
    pub fn new(chessboard: Chessboard, depth: u32, color: u8, heuristics: HeuristicParams, available_memory_mb: usize) -> Self {
        // Use all available memory specifically for the transposition table
        let tt_memory = available_memory_mb;

        let transposition_table = TranspositionTable::new(tt_memory.max(1));
        let zobrist_table = ZobristTable::new(chessboard);

        let killer_moves = HashMap::new();
        Engine {
            depth,
            color,
            transposition_table,
            killer_moves,
            heuristics,
            zobrist_table,
        }
    }

    pub fn get_best_move(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        self.start_single_search(chessboard)
    }

    fn start_single_search(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        let start = std::time::Instant::now();

        let moves = generate_moves(chessboard, self.color);
        // println!("{} moves available for engine", moves.len());
        if moves.is_empty() {
            return None;
        }

        let mut best_move: Option<Move> = None;
        let mut best_score = MIN_SCORE;

        let mut last_time = 0u64;
        let mut reached_depth = 0;
        // Iterative deepening loop
        for current_depth in 1..=self.depth {
            if start.elapsed().as_millis() as u64 > TIME_CUTOFF {
                // println!("Time cutoff reached, stopping search at depth {}", current_depth);
                break; // Basic time management
            }

            let result = alpha_beta(
                current_depth,
                MIN_SCORE,
                MAX_SCORE,
                self.color,
                self.color,
                true,
                chessboard,
                &mut self.transposition_table,
                &mut self.killer_moves,
                &self.heuristics,
                &start
            );

            // If we timed out inside alpha_beta or didn't get a result, don't overwrite best_move
            if start.elapsed().as_millis() as u64 > TIME_CUTOFF {
                if best_move.is_none() && result.best_move.is_some() {
                    best_move = result.best_move;
                }
                break;
            }

            if result.best_move.is_some() {
                best_move = result.best_move;
            } else if best_move.is_none() && !moves.is_empty() {
                // Fallback to first available move if alpha-beta returns none
                best_move = Some(moves[0]);
            }
            best_score = result.score;

            last_time = start.elapsed().as_millis() as u64;
            reached_depth = current_depth;
        }
        println!("Engine chose move {:?} with score {} in {} ms, depth {}", best_move, best_score, last_time, reached_depth);
        best_move
    }
}

// go to the wikipedia page if you want to understand this
fn alpha_beta(
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    color_to_move: u8,
    root_color: u8,
    maximizing_player: bool,
    chessboard: &mut Chessboard,
    transposition_table: &mut TranspositionTable,
    killer_moves: &mut HashMap<Move, i32>,
    heuristics_params: &HeuristicParams,
    start_time: &std::time::Instant,
) -> Result {
    if depth == 0 {
        let score = evaluate(chessboard, root_color, heuristics_params);
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
    let mut moves = generate_moves(chessboard, color_to_move);
    if moves.len() == 0 {
        return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
    }
    // Order moves based on scores (e.g., killer moves, history heuristics)
    moves.sort_by_cached_key(|mv| {
        if let Some(score) = killer_moves.get(mv) {
            -*score // Prefer killer moves (higher scores first)
        } else {
            0 // Default score for icons moves
        }
    });
    for mov in moves {
        // Enforce time abort during deep branches
        if start_time.elapsed().as_millis() as u64 > TIME_CUTOFF {
            return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
        }

        chessboard.make_move(mov);
        let result = alpha_beta(
            depth - 1,
            alpha,
            beta,
            color_to_move ^ 1,
            root_color,
            !maximizing_player,
            chessboard,
            transposition_table,
            killer_moves,
            heuristics_params,
            start_time,
        );
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
                //if we caused a cutoff, add this move to killer moves so that we skip the branch as soon as possible if we find it again
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
        let entry_size = size_of::<Option<Entry>>();
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
