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

use std::time::{Duration, Instant};
use crate::chessboard::Chessboard;
use crate::{BLACK, WHITE};
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::heuristics::{Heuristics, HeuristicParams};

const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;


const TIME_CUTOFF: u64 = 2000;

#[derive(Clone)]
pub struct Engine {
    depth: i32,
    color: i8,
    transposition_table: TranspositionTable,
    killer_moves: HashMap<Move, i32>,
    heuristics: HeuristicParams,
    benchmarking: bool,
}

impl Engine {
    pub fn new_single(depth: i32, color: i8, heuristics: HeuristicParams, available_memory_mb: usize, benchmarking: bool) -> Self {
        let tt_memory = available_memory_mb;

        let transposition_table = TranspositionTable::new(tt_memory.max(1));

        let killer_moves = HashMap::new();
        Engine {
            depth,
            color,
            transposition_table,
            killer_moves,
            heuristics,
            benchmarking,
        }
    }

    pub fn get_best_move(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        self.start_single_search(chessboard)
    }

    fn start_single_search(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        let start = Instant::now();
        let mut benchmark_stats = BenchmarkStats::default();

        let mut moves = generate_moves(chessboard, self.color);
        // println!("{} moves available for engine", moves.len());
        if moves.is_empty() {
            return None;
        }

        let mut best_move: Option<Move> = None;
        let mut best_score = MIN_SCORE;

        let mut last_time = 0u64;
        // Iterative deepening loop
        for current_depth in 1..=self.depth {
            if start.elapsed().as_millis() as u64 > TIME_CUTOFF {
                println!("Time cutoff reached, stopping search at depth {}", current_depth);
                break; // Basic time management
            }

            let result = alpha_beta(
                current_depth,
                MIN_SCORE,
                MAX_SCORE,
                self.color,
                true,
                chessboard,
                &mut self.transposition_table,
                &mut self.killer_moves,
                &self.heuristics,
                &start,
                self.benchmarking,
                &mut benchmark_stats,
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

            println!("Depth: {}, Score: {}, Time: {}ms", current_depth, best_score, last_time);
        }

        if self.benchmarking {
            benchmark_stats.print_averages();
        }

        println!("Score: {}, Time: {}ms", best_score, last_time);
        best_move
    }
}

// go to the wikipedia page if you want to understand this
fn alpha_beta(
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    color: i8,
    maximizing_player: bool,
    chessboard: &mut Chessboard,
    transposition_table: &mut TranspositionTable,
    killer_moves: &mut HashMap<Move, i32>,
    heuristics_params: &HeuristicParams,
    start_time: &Instant,
    benchmarking: bool,
    benchmark_stats: &mut BenchmarkStats) -> Result {

    if depth == 0 {
        let step_start = Instant::now();
        let eval_color = if maximizing_player { color } else { -color };
        let score = evaluate(chessboard, eval_color, heuristics_params);
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::DepthZeroEval, step_start.elapsed());
        /*println!("Reached end of depth");
           chessboard.print_board();*/
        return Result::new(score, None);
    }

    let step_start = Instant::now();
    let board_key: u64 = chessboard.get_hash();
    record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::BoardHash, step_start.elapsed());
    // Check transposition table
    let step_start = Instant::now();
    if let Some(entry) = transposition_table.get(board_key) {
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::TranspositionLookup, step_start.elapsed());
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
    } else {
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::TranspositionLookup, step_start.elapsed());
    }
    let mut best_score = if maximizing_player { MIN_SCORE } else { MAX_SCORE };
    let mut best_move = None;
    let step_start = Instant::now();
    let mut moves = generate_moves(chessboard, color);
    record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::MoveGeneration, step_start.elapsed());
    let step_start = Instant::now();
    if moves.len() == 0 {
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::EmptyMoveCheck, step_start.elapsed());
        return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
    }
    record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::EmptyMoveCheck, step_start.elapsed());
    // Order moves based on killer moves
    let step_start = Instant::now();
    moves.sort_by_cached_key(|mv| {
        if let Some(score) = killer_moves.get(mv) {
            -*score // Prefer killer moves (higher scores first)
        } else {
            0 // Default score for icons moves
        }
    });
    record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::MoveOrdering, step_start.elapsed());
    for mov in moves {
        // time cutoff
        let step_start = Instant::now();
        if start_time.elapsed().as_millis() as u64 > TIME_CUTOFF {
            record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::TimeCutoffCheck, step_start.elapsed());
            return Result::new(if maximizing_player { MIN_SCORE } else { MAX_SCORE }, None);
        }
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::TimeCutoffCheck, step_start.elapsed());

        let step_start = Instant::now();
        chessboard.make_move(mov);
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::MakeMove, step_start.elapsed());
        let result = alpha_beta(
            depth - 1,
            alpha,
            beta,
            -color,
            !maximizing_player,
            chessboard,
            transposition_table,
            killer_moves,
            heuristics_params,
            start_time,
            benchmarking,
            benchmark_stats,
        );
        let step_start = Instant::now();
        chessboard.undo_move();
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::UndoMove, step_start.elapsed());

        let step_start = Instant::now();
        let score = result.score;
        let mut cutoff_triggered = false;
        if maximizing_player && score > best_score {
            best_move = Some(mov);
            best_score = score;
            alpha = std::cmp::max(alpha, score);
            if beta <= alpha {
                let cutoff_start = Instant::now();
                transposition_table.insert(Entry {
                    hash: board_key,
                    score: best_score,
                    depth,
                    best_move,
                    bound_type: LowerBound});
                //if we caused a cutoff, add this move to killer moves so that we skip the branch as soon as possible if we find it again
                killer_moves.insert(best_move.unwrap(), best_score);
                record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::CutoffStorage, cutoff_start.elapsed());
                cutoff_triggered = true;
            }
        } else if !maximizing_player && score < best_score {
            best_move = Some(mov);
            best_score = score;
            beta = std::cmp::min(beta, score);
            if beta <= alpha {
                let cutoff_start = Instant::now();
                transposition_table.insert(Entry {
                    hash: board_key,
                    score: best_score,
                    depth,
                    best_move,
                    bound_type: UpperBound});
                killer_moves.insert(best_move.unwrap(), best_score);
                record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::CutoffStorage, cutoff_start.elapsed());
                cutoff_triggered = true;
            }
        }
        record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::ScoreUpdate, step_start.elapsed());
        if cutoff_triggered {
            break;
        }
    }
    let step_start = Instant::now();
    let bound_type = if best_score <= alpha {
        UpperBound
    } else {
        LowerBound
    };
    transposition_table.insert(Entry {
        hash: board_key,
        score: best_score,
        depth,
        best_move,
        bound_type});
    record_benchmark(benchmarking, benchmark_stats, BenchmarkStep::FinalStorage, step_start.elapsed());
    Result::new(best_score, best_move)
}
pub fn evaluate(position: &Chessboard, color: i8, params: &HeuristicParams) -> i32 {
    let mut score = 0;
    score += material(position, color);
    score += evaluate_heuristics(position, color, params);
    score
}
pub fn evaluate_heuristics(position: &Chessboard, color: i8, h: &HeuristicParams) -> i32 {
    let mut score: i32 = 0;
    let heuristics_eval = Heuristics::new(position, color);
    score += heuristics_eval.two_middle_pawns() * h.two_middle_pawns_weight;
    score += heuristics_eval.castling() * h.castling_weight;
    score += heuristics_eval.knight_outpost() * h.knight_outpost_weight;
    score += heuristics_eval.development() * h.development_weight;
    score += heuristics_eval.mobility() * h.mobility_weight;
    score
}
pub fn material(position: &Chessboard, for_color: i8) -> i32 {
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

#[derive(Debug, Clone, Copy)]
enum BenchmarkStep {
    DepthZeroEval,
    BoardHash,
    TranspositionLookup,
    MoveGeneration,
    EmptyMoveCheck,
    MoveOrdering,
    TimeCutoffCheck,
    MakeMove,
    UndoMove,
    ScoreUpdate,
    CutoffStorage,
    FinalStorage,
}

#[derive(Debug, Clone, Default)]
struct BenchmarkStats {
    depth_zero_eval_count: u64,
    depth_zero_eval_total_ns: u128,
    board_hash_count: u64,
    board_hash_total_ns: u128,
    transposition_lookup_count: u64,
    transposition_lookup_total_ns: u128,
    move_generation_count: u64,
    move_generation_total_ns: u128,
    empty_move_check_count: u64,
    empty_move_check_total_ns: u128,
    move_ordering_count: u64,
    move_ordering_total_ns: u128,
    time_cutoff_check_count: u64,
    time_cutoff_check_total_ns: u128,
    make_move_count: u64,
    make_move_total_ns: u128,
    undo_move_count: u64,
    undo_move_total_ns: u128,
    score_update_count: u64,
    score_update_total_ns: u128,
    cutoff_storage_count: u64,
    cutoff_storage_total_ns: u128,
    final_storage_count: u64,
    final_storage_total_ns: u128,
}

impl BenchmarkStats {
    fn record(&mut self, step: BenchmarkStep, elapsed: Duration) {
        let elapsed_ns = elapsed.as_nanos();
        match step {
            BenchmarkStep::DepthZeroEval => {
                self.depth_zero_eval_count += 1;
                self.depth_zero_eval_total_ns += elapsed_ns;
            }
            BenchmarkStep::BoardHash => {
                self.board_hash_count += 1;
                self.board_hash_total_ns += elapsed_ns;
            }
            BenchmarkStep::TranspositionLookup => {
                self.transposition_lookup_count += 1;
                self.transposition_lookup_total_ns += elapsed_ns;
            }
            BenchmarkStep::MoveGeneration => {
                self.move_generation_count += 1;
                self.move_generation_total_ns += elapsed_ns;
            }
            BenchmarkStep::EmptyMoveCheck => {
                self.empty_move_check_count += 1;
                self.empty_move_check_total_ns += elapsed_ns;
            }
            BenchmarkStep::MoveOrdering => {
                self.move_ordering_count += 1;
                self.move_ordering_total_ns += elapsed_ns;
            }
            BenchmarkStep::TimeCutoffCheck => {
                self.time_cutoff_check_count += 1;
                self.time_cutoff_check_total_ns += elapsed_ns;
            }
            BenchmarkStep::MakeMove => {
                self.make_move_count += 1;
                self.make_move_total_ns += elapsed_ns;
            }
            BenchmarkStep::UndoMove => {
                self.undo_move_count += 1;
                self.undo_move_total_ns += elapsed_ns;
            }
            BenchmarkStep::ScoreUpdate => {
                self.score_update_count += 1;
                self.score_update_total_ns += elapsed_ns;
            }
            BenchmarkStep::CutoffStorage => {
                self.cutoff_storage_count += 1;
                self.cutoff_storage_total_ns += elapsed_ns;
            }
            BenchmarkStep::FinalStorage => {
                self.final_storage_count += 1;
                self.final_storage_total_ns += elapsed_ns;
            }
        }
    }

    fn print_averages(&self) {
        println!("\n=== Alpha-beta benchmarking averages ===");
        Self::print_metric("depth_zero_eval", self.depth_zero_eval_total_ns, self.depth_zero_eval_count);
        Self::print_metric("board_hash", self.board_hash_total_ns, self.board_hash_count);
        Self::print_metric("transposition_lookup", self.transposition_lookup_total_ns, self.transposition_lookup_count);
        Self::print_metric("move_generation", self.move_generation_total_ns, self.move_generation_count);
        Self::print_metric("empty_move_check", self.empty_move_check_total_ns, self.empty_move_check_count);
        Self::print_metric("move_ordering", self.move_ordering_total_ns, self.move_ordering_count);
        Self::print_metric("time_cutoff_check", self.time_cutoff_check_total_ns, self.time_cutoff_check_count);
        Self::print_metric("make_move", self.make_move_total_ns, self.make_move_count);
        Self::print_metric("undo_move", self.undo_move_total_ns, self.undo_move_count);
        Self::print_metric("score_update", self.score_update_total_ns, self.score_update_count);
        Self::print_metric("cutoff_storage", self.cutoff_storage_total_ns, self.cutoff_storage_count);
        Self::print_metric("final_storage", self.final_storage_total_ns, self.final_storage_count);
    }

    fn print_metric(name: &str, total_ns: u128, count: u64) {
        if count == 0 {
            println!("  {:<22}: n/a (0 samples)", name);
            return;
        }

        let avg_ns = total_ns / count as u128;
        let avg_us = avg_ns as f64 / 1_000.0;
        println!("  {:<22}: {:>9.3} µs over {:>8} samples", name, avg_us, count);
    }
}

fn record_benchmark(benchmarking: bool, benchmark_stats: &mut BenchmarkStats, step: BenchmarkStep, elapsed: Duration) {
    if benchmarking {
        benchmark_stats.record(step, elapsed);
    }
}

