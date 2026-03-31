use serde::{Deserialize, Serialize};

/// Configuration for tuning hyperparameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningConfig {
    /// Number of generations to evolve
    pub num_generations: u32,

    /// Population size per generation
    pub population_size: u32,

    /// Fraction of population to keep as survivors (e.g., 0.2 = top 20%)
    pub survival_rate: f32,

    /// Number of mutant copies per survivor
    pub offspring_per_survivor: u32,

    /// Standard deviation of mutation (Gaussian noise applied to parameters)
    pub mutation_std_dev: f32,

    /// Search depth for game evaluations
    pub search_depth: i32,

    /// Number of games each variant plays (tournament size)
    pub games_per_matchup: u32,

    /// Time limit per game (in seconds)
    pub time_per_game: u64,

    /// Number of parallel threads for tournament games
    pub num_threads: usize,

    /// Whether to enable detailed logging
    pub verbose: bool,

    /// Output directory for results
    pub output_dir: String,

    /// Available memory limit for the tournament matching (MB)
    pub available_memory: usize,
}

impl Default for TuningConfig {
    fn default() -> Self {
        Self {
            num_generations: 2,
            population_size: 4,
            survival_rate: 0.2,
            offspring_per_survivor: 5,
            mutation_std_dev: 2.0,
            search_depth: 12,
            games_per_matchup: 2,
            time_per_game: 30,
            num_threads: 12,
            verbose: true,
            output_dir: "results".to_string(),
            available_memory: 20480,
        }
    }
}

impl TuningConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.population_size < 4 {
            return Err("Population size must be >= 4".to_string());
        }
        if self.survival_rate <= 0.0 || self.survival_rate > 1.0 {
            return Err("Survival rate must be in (0, 1]".to_string());
        }
        if self.search_depth < 2 || self.search_depth > 20 {
            return Err("Search depth should be between 2 and 20".to_string());
        }
        Ok(())
    }

    /// Calculate number of survivors
    pub fn num_survivors(&self) -> u32 {
        std::cmp::max((self.population_size as f32 * self.survival_rate) as u32, 1)
    }
}
