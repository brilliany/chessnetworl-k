use config::{FileFormat, Config, File, Source};
use config::ValueKind::I128;
use crate::engine::Engine;

mod chessboard;
mod movegenerator;
mod r#move;
mod engine;
mod heuristics;
#[path = "api/server.rs"] mod server;

const WHITE: i8 = 1;
const BLACK: i8 = -1;
const PAWN: i8 = 1;
const KNIGHT: i8 = 2;
const BISHOP: i8 = 3;
const ROOK: i8 = 4;
const QUEEN: i8 = 5;
const KING: i8 = 6;
const EMPTY: i8 = 0;

fn main() {
    setup();
}

// WORKING CODE:
/*
//multithreading test: count to COUNT using as many threads as there are cores

    const COUNT: u64 = 10000; /*u64::MAX;*/
    //use arc to share counter between threads
    let counter = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let mut thread_counter = 0;
    let mut threads = Vec::new();
    let num_cpus = num_cpus::get();
    let mut num_threads = num_cpus;

    //if num_cpus is odd, use one less thread
    if num_cpus % 2 == 1 {
        num_threads -= 1;
    }

    //create threads
    for _ in 0..num_threads {
        let counter = counter.clone();
        let thread = std::thread::spawn(move || {
            //while counter is less than COUNT, increment counter
            while counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < COUNT {
                thread_counter += 1;
            }
        });
        threads.push(thread);
    }

    let time = std::time::Instant::now();
    //wait for threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
    let time = time.elapsed();
    println!("time: {}", time.as_nanos());


    println!("counter: {:?}", counter);
    println!("thread_counter: {}", thread_counter);
*/


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