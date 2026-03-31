use crate::population::Population;
use crate::tournament::Tournament;
use crate::config::TuningConfig;
use crate::stats::GenerationStats;
use rand::Rng;

/// Main evolution loop coordinator
pub struct EvolutionStrategy {
    config: TuningConfig,
    tournament: Tournament,
}

impl EvolutionStrategy {
    pub fn new(config: TuningConfig) -> Self {
        let tournament = Tournament::new(config.clone());
        Self { config, tournament }
    }

    /// Run the full evolution process
    pub fn run(&self, rng: &mut impl Rng) -> (Vec<GenerationStats>, Option<crate::variant::EngineVariant>) {
        let mut population = Population::initialize(&self.config, rng);
        let mut generation_stats = Vec::new();

        println!("Starting evolution: {} generations, {} population size",
            self.config.num_generations, self.config.population_size);

        for gen in 0..self.config.num_generations {
            if self.config.verbose {
                println!("\n=== Generation {} ===", gen);
            }

            // Reset statistics for this generation
            population.reset_statistics();

            // Run tournament
            let _results = self.tournament.run_round_robin(population.variants_mut());

            // Sort by fitness
            population.sort_by_fitness();

            // Record statistics
            let stats = GenerationStats::from_population(&population, gen);
            generation_stats.push(stats.clone());

            if self.config.verbose {
                println!("{}", stats);
            }

            // Selection and mutation
            let num_survivors = self.config.num_survivors() as usize;
            let survivors = population.top_n(num_survivors);

            // Evolve to next generation
            population.evolve_generation(&survivors, &self.config, rng);
        }

        let best = population.best_variant().cloned();
        (generation_stats, best)
    }
}
