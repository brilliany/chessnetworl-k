use crate::variant::{EngineVariant, HeuristicParams};
use crate::config::TuningConfig;
use rand::Rng;

/// Manages the population of engine variants
pub struct Population {
    variants: Vec<EngineVariant>,
    next_id: u32,
}

impl Population {
    /// Create initial random population
    pub fn initialize(config: &TuningConfig, rng: &mut impl Rng) -> Self {
        let mut variants = Vec::with_capacity(config.population_size as usize);

        // Add baseline (default parameters)
        variants.push(EngineVariant::new(0, HeuristicParams::default()));

        // Add random variants
        for i in 1..config.population_size {
            variants.push(EngineVariant::new(i, HeuristicParams::random(rng)));
        }

        Self {
            variants,
            next_id: config.population_size,
        }
    }

    /// Get all variants (mutable)
    pub fn variants_mut(&mut self) -> &mut Vec<EngineVariant> {
        &mut self.variants
    }

    /// Get all variants (immutable)
    pub fn variants(&self) -> &[EngineVariant] {
        &self.variants
    }

    /// Get variant by ID
    pub fn get_by_id(&self, id: u32) -> Option<&EngineVariant> {
        self.variants.iter().find(|v| v.id == id)
    }

    /// Sort by fitness (descending)
    pub fn sort_by_fitness(&mut self) {
        self.variants.sort_by(|a, b| {
            b.fitness_score
                .partial_cmp(&a.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Get top N variants (survivors)
    pub fn top_n(&self, n: usize) -> Vec<EngineVariant> {
        self.variants.iter().take(n).cloned().collect()
    }

    /// Create next generation from survivors
    pub fn evolve_generation(
        &mut self,
        survivors: &[EngineVariant],
        config: &TuningConfig,
        rng: &mut impl Rng,
    ) {
        let mut new_generation = Vec::with_capacity(config.population_size as usize);

        // Keep survivors
        for survivor in survivors {
            new_generation.push(survivor.clone());
        }

        // Generate offspring
        while new_generation.len() < config.population_size as usize {
            let survivor = &survivors[rng.gen_range(0..survivors.len())];
            let mutant = survivor.mutate(
                self.next_id,
                config.mutation_std_dev,
                rng,
            );
            self.next_id += 1;
            new_generation.push(mutant);
        }

        // Trim to exact population size
        new_generation.truncate(config.population_size as usize);
        self.variants = new_generation;
    }

    /// Reset game statistics for new tournament round
    pub fn reset_statistics(&mut self) {
        for variant in &mut self.variants {
            variant.win_rate = 0.0;
            variant.draw_count = 0;
            variant.loss_count = 0;
            variant.games_played = 0;
            variant.fitness_score = 0.0;
        }
    }

    pub fn size(&self) -> usize {
        self.variants.len()
    }

    pub fn best_variant(&self) -> Option<&EngineVariant> {
        self.variants.first()
    }
}
