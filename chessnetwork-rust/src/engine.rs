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
use crate::engine::BoundType::{LowerBound, UpperBound};
use crate::movegenerator::generate_moves;
use crate::r#move::Move;
use crate::heuristics::Heuristics;
use crate::pieces::Color::White;

const MIN_SCORE: i32 = i32::MIN + 30_000;
const MAX_SCORE: i32 = i32::MAX - 30_000;

#[derive(Clone)]
enum Mode {
    Continuous,
    Single,
}

#[derive(Clone)]
pub(crate) struct Engine {
    mode: Mode,
    depth: i32,
    color: i8,
    search_tx: Arc<Mutex<Option<mpsc::Sender<Vec<Move>>>>>,
    confidence: i32,
    max_score: i32,
    min_score: i32,
    transposition_table: Arc<Mutex<TranspositionTable>>,
    killer_moves: Arc<Mutex<HashMap<Move, i32>>>,
}

impl Engine {
    pub fn new_continuous(depth: i32, color: i8, start_position: Chessboard) -> Self {
        let transposition_table = Arc::new(Mutex::new(TranspositionTable::new()));
        let killer_moves = Arc::new(Mutex::new(HashMap::new()));
        let search_tx = Arc::new(Mutex::new(None));
        let search_tx2 = search_tx.clone();

        let engine = Engine {
            mode: Mode::Continuous,
            depth,
            color,
            search_tx,
            confidence: 0,
            max_score: i32::MAX,
            min_score: i32::MIN,
            transposition_table,
            killer_moves,
        };
        let mut thread_engine = engine.clone();
        thread::spawn(move || thread_engine.start_multi_search(search_tx2, start_position));
        engine
    }

    pub fn new_single(depth: i32, color: i8) -> Self {
        let transposition_table = Arc::new(Mutex::new(TranspositionTable::new()));
        let killer_moves = Arc::new(Mutex::new(HashMap::new()));
        Engine {
            mode: Mode::Single,
            depth,
            color,
            search_tx: Arc::new(Mutex::new(None)),
            confidence: 0,
            max_score: i32::MAX,
            min_score: i32::MIN,
            transposition_table,
            killer_moves,
        }
    }

    pub fn get_best_move(&mut self, chessboard: &mut Chessboard) -> Option<Move> {
        match &self.mode {
            Mode::Continuous => {
                let (tx, rx) = mpsc::channel();
                self.search_tx.lock().unwrap().replace(tx);
                rx.recv().ok().and_then(|moves| moves.into_iter().next())
            }
            Mode::Single => Some(self.start_single_search(chessboard)),
        }
    }


    fn start_single_search(&mut self, chessboard: &mut Chessboard) -> Move {
        let start = std::time::Instant::now();

        let moves = generate_moves(chessboard, self.color);
        let chunk_size = moves.len() / num_cpus::get();

        // Divide the moves into chunks for parallel processing
        let chunks: Vec<Vec<_>> = moves.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();

        // Create shared references to the data structures
        let shared_killer_moves = Arc::new(Mutex::new(self.killer_moves.lock().unwrap().clone()));
        let shared_transposition_table = Arc::new(Mutex::new((*self.transposition_table.lock().unwrap()).clone()));
        let shared_chessboard = Arc::new(Mutex::new(chessboard.clone()));
        let shared_color = self.color;
        let shared_depth = self.depth;
        let shared_max_score = self.max_score;
        let shared_min_score = self.min_score;

        // Spawn threads to process moves in parallel
        let handles: Vec<_> = chunks
            .into_iter()
            .map(|chunk| {
                let thread_chessboard = Arc::clone(&shared_chessboard);
                let thread_killer_moves = Arc::clone(&shared_killer_moves);
                let thread_transposition_table = Arc::clone(&shared_transposition_table);

                thread::spawn(move || {
                    // Use shared references to the killer_moves and transposition_table
                    let mut killer_moves = thread_killer_moves;
                    let mut transposition_table = thread_transposition_table;

                    let search = Search {
                        depth: shared_depth,
                        maximizing_color: shared_color,
                        max_score: shared_max_score,
                        min_score: shared_min_score,
                    };

                    let mut alpha = shared_min_score;
                    let beta = shared_max_score;

                    let mut best_move = None;
                    let mut best_score = shared_min_score;

                    for mv in chunk {
                        let mut chessboard = thread_chessboard.lock().unwrap();
                        chessboard.make_move(mv);

                        let result = search.alpha_beta(
                            shared_depth - 1,
                            alpha,
                            beta,
                            shared_color,
                            true,
                            &mut *chessboard,
                            true,
                            &mut transposition_table.lock().unwrap(),
                            &mut killer_moves.lock().unwrap(),
                        );

                        chessboard.undo_move();

                        if let Some(new_best_move) = result.best_move {
                            best_move = Some(new_best_move);
                        }

                        if result.score == shared_max_score {
                            break;
                        }

                        best_score = result.score;

                        if best_score > alpha {
                            alpha = best_score;
                        }
                    }

                    let end = std::time::Instant::now();
                    println!("Thread {:?} time: {:?}", thread::current().id(), end - start);
                    (best_move, best_score)
                })
            })
            .collect();

        // Collect results from threads and find the best move
        let (mut best_move, mut best_score) = (None, self.min_score);

        for handle in handles {
            let (move_result, score_result) = handle.join().unwrap();

            if score_result > best_score {
                best_score = score_result;
                best_move = move_result;
            }
        }

        let best = best_move.expect("No best move found");

        let end = std::time::Instant::now();
        println!("Time: {:?}", end - start);

        best
    }

    fn start_multi_search(&mut self, search_tx: Arc<Mutex<Option<mpsc::Sender<Vec<Move>>>>>, mut chessboard: Chessboard) {
        // let search = Search {
        //     depth: self.depth,
        //     maximizing_color: self.color,
        //     max_score: self.max_score,
        //     min_score: self.min_score,
        //     transposition_table: self.transposition_table.clone(),
        //     killer_moves: self.killer_moves.clone(),
        // };
        //
        // loop {
        //     let mut alpha = self.min_score;
        //     let mut beta = self.max_score;
        //
        //     let mut best_moves = Vec::new();
        //
        //     let start_time = std::time::Instant::now();
        //
        //     // perform iterative deepening search
        //     for current_depth in 1..=self.depth {
        //         let result = search.alpha_beta(
        //             current_depth,
        //             alpha,
        //             beta,
        //             self.color,
        //             true,
        //             &mut chessboard,
        //             true,
        //
        //         );
        //
        //         if let Some(best_move) = result.best_move {
        //             best_moves.push(best_move);
        //         }
        //
        //         //todo add change in alpha beta window here
        //
        //         // check if search was interrupted
        //         if let Some(search_tx) = &*search_tx.lock().unwrap() {
        //             if search_tx.send(best_moves.clone()).is_err() {
        //                 return;
        //             }
        //         }
        //
        //         // check if search should be stopped early
        //         if start_time.elapsed().as_secs() > 10 {
        //             break;
        //         }
        //     }
        //
        //     // send best moves to listener
        //     if let Some(search_tx) = &*search_tx.lock().unwrap() {
        //         search_tx.send(best_moves).ok();
        //     }
        // }
    }
}

pub(crate) struct Search {
    depth: i32,
    maximizing_color: i8,
    max_score: i32,
    min_score: i32,
}

/** Board to key function
       * creates a string that represents the board and can be undone to get the board back
 */
fn board_to_key(board: & Chessboard) -> String {
    let mut key = String::new();
    for i in 0..64 {
        let piece = board.get_piece_at(i);
        if piece == Empty {
            key.push_str("0");
        } else {
            key.push_str(&piece.to_string());
        }
    }
    key
}
/** Key to board function
       * undoes the board to key function
 */
fn key_to_board(key: String) -> Chessboard {
    let mut board = Chessboard::default();
    board.init();
    for i in 0..64 {
        let piece = key.chars().nth(i).unwrap() as i8;
        board.set_piece(i as u8, piece);
    }
    board
}
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
        // println!("Time taken to analyze moves: {} ms", start_analysis_time.elapsed().as_millis());
        // println!("Total time taken: {} ms", start_time.elapsed().as_millis());
        Result::new(best_score, best_move)
    }

    pub(crate) fn evaluate(position: &Chessboard, color: i8) -> i32 {
        let mut score = 0;
        //timer
        let start_time = std::time::Instant::now();
        score += Self::material(position, color);
        let material_time = start_time.elapsed().as_nanos();
        // println!("Score after material: {}", score);
        score += Self::heuristics(position, color);
        let heuristics_time = start_time.elapsed().as_nanos() - material_time;
        // println!("Score after heuristics: {}", score);

        // println!("Time taken to evaluate material: {} ns", material_time);
        // println!("Time taken to evaluate heuristics: {} ns", heuristics_time);
        score
    }

    fn heuristics(position: &Chessboard, color: i8) -> i32 {
        let mut score: i32 = 0;
        let heuristics = Heuristics::new(position, color);
        score += heuristics.two_middle_pawns();
        // println!("Score after two middle pawns: {}", score);
        score += heuristics.castling();
        // println!("Score after castling: {}", score);
        score += heuristics.knight_outpost();
        // println!("Score after knight outpost: {}", score);
        score += heuristics.development();
        // println!("Score after development: {}", score);
        score += heuristics.mobility();
        // println!("Score after mobility: {}", score);

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
