use chessnetwork_tuning::{EvolutionStrategy, TuningConfig};
use rand::thread_rng;
use std::fs;
use std::path::Path;

fn main() {
    let default_memory = chessnetwork_core::load_available_memory_from_config("config.yml");

    let mut config = TuningConfig::default();
    config.num_generations = 2;
    config.population_size = 4;
    config.survival_rate = 0.2;
    config.mutation_std_dev = 2.0;
    config.search_depth = 12;
    config.num_threads = 12;
    config.output_dir = "results".to_string();
    config.verbose = true;
    config.available_memory = default_memory;

    // Validate config
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }

    // Create output directory
    if !Path::new(&config.output_dir).exists() {
        fs::create_dir_all(&config.output_dir).expect("Failed to create output directory");
    }

    println!("Configuration:");
    println!("  Generations: {}", config.num_generations);
    println!("  Population: {}", config.population_size);
    println!("  Survival rate: {}", config.survival_rate);
    println!("  Mutation std dev: {}", config.mutation_std_dev);
    println!("  Search depth: {}", config.search_depth);
    println!("  Threads: {}", config.num_threads);
    println!("  Available Memory: {}MB", config.available_memory);

    // Initialize rayon thread pool
    if let Err(e) = rayon::ThreadPoolBuilder::new().num_threads(config.num_threads).build_global() {
        eprintln!("Failed to initialize thread pool: {}", e);
    }

    let mut rng = thread_rng();
    let strategy = EvolutionStrategy::new(config.clone());

    let start_time = std::time::Instant::now();
    let (stats, best_variant) = strategy.run(&mut rng);
    let elapsed = start_time.elapsed();

    // Save results
    let results_file = format!("{}/results.json", config.output_dir);

    #[derive(serde::Serialize)]
    struct RunResults<'a> {
        stats: &'a Vec<chessnetwork_tuning::GenerationStats>,
        best_variant: &'a Option<chessnetwork_tuning::EngineVariant>,
    }

    let run_results = RunResults {
        stats: &stats,
        best_variant: &best_variant,
    };

    let json = serde_json::to_string_pretty(&run_results).expect("Failed to serialize results");
    fs::write(&results_file, json).expect("Failed to write results file");

    println!("\n=== Evolution Complete ===");
    println!("Total time: {:.2}s", elapsed.as_secs_f32());
    println!("Results saved to: {}", results_file);

    if let Some(best) = stats.last() {
        println!("Final best fitness: {:.4}", best.best_fitness);
    }
}
