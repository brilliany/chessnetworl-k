//! Reinforcement Learning-based tuning module for chess engine heuristics
//! 
//! Simple evolutionary algorithm using self-play against mutated variants.

pub mod variant;
pub mod population;
pub mod evolution;
pub mod tournament;
pub mod config;
pub mod stats;

// Re-export main types
pub use variant::EngineVariant;
pub use population::Population;
pub use evolution::EvolutionStrategy;
pub use config::TuningConfig;
pub use stats::GenerationStats;
