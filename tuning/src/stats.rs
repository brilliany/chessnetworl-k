use crate::population::Population;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Statistics for a single generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationStats {
    pub generation: u32,
    pub best_fitness: f32,
    pub average_fitness: f32,
    pub worst_fitness: f32,
    pub best_variant_id: u32,
    pub population_size: u32,
}

impl GenerationStats {
    pub fn from_population(population: &Population, generation: u32) -> Self {
        let variants = population.variants();

        let fitnesses: Vec<f32> = variants.iter().map(|v| v.fitness_score).collect();

        let best_fitness = fitnesses.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let worst_fitness = fitnesses.iter().cloned().fold(f32::INFINITY, f32::min);
        let average_fitness = fitnesses.iter().sum::<f32>() / fitnesses.len() as f32;

        let best_variant_id = population
            .best_variant()
            .map(|v| v.id)
            .unwrap_or(0);

        Self {
            generation,
            best_fitness,
            average_fitness,
            worst_fitness,
            best_variant_id,
            population_size: variants.len() as u32,
        }
    }

    /// Save to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl fmt::Display for GenerationStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gen {}: Best={:.4}, Avg={:.4}, Worst={:.4} (id={})",
            self.generation,
            self.best_fitness,
            self.average_fitness,
            self.worst_fitness,
            self.best_variant_id
        )
    }
}
