use chessnetwork_core::HeuristicParams as CoreHeuristicParams;
use serde::{Deserialize, Serialize};
use rand::Rng;

/// Representation of a single heuristic parameter set (engine variant)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicParams {
    pub two_middle_pawns_weight: i32,
    pub castling_weight: i32,
    pub knight_outpost_weight: i32,
    pub development_weight: i32,
    pub mobility_weight: i32,
}

impl Into<CoreHeuristicParams> for &HeuristicParams {
    fn into(self) -> CoreHeuristicParams {
        CoreHeuristicParams {
            two_middle_pawns_weight: self.two_middle_pawns_weight,
            castling_weight: self.castling_weight,
            knight_outpost_weight: self.knight_outpost_weight,
            development_weight: self.development_weight,
            mobility_weight: self.mobility_weight,
        }
    }
}

impl HeuristicParams {
    /// Create default parameters (same as current hardcoded values)
    pub fn default() -> Self {
        Self {
            two_middle_pawns_weight: 10,
            castling_weight: 10,
            knight_outpost_weight: 10,
            development_weight: 10,
            mobility_weight: 10,
        }
    }

    /// Create random parameters within reasonable bounds
    pub fn random(rng: &mut impl Rng) -> Self {
        Self {
            two_middle_pawns_weight: rng.gen_range(1..50),
            castling_weight: rng.gen_range(1..50),
            knight_outpost_weight: rng.gen_range(1..50),
            development_weight: rng.gen_range(1..50),
            mobility_weight: rng.gen_range(1..50),
        }
    }

    /// Apply Gaussian mutation
    pub fn mutate(&self, std_dev: f32, rng: &mut impl Rng) -> Self {
        use rand_distr::{Normal, Distribution};

        let normal = Normal::new(0.0, std_dev as f64).unwrap();

        Self {
            two_middle_pawns_weight: clamp(
                (self.two_middle_pawns_weight as f32 + normal.sample(rng) as f32) as i32,
                1,
                100,
            ),
            castling_weight: clamp(
                (self.castling_weight as f32 + normal.sample(rng) as f32) as i32,
                1,
                100,
            ),
            knight_outpost_weight: clamp(
                (self.knight_outpost_weight as f32 + normal.sample(rng) as f32) as i32,
                1,
                100,
            ),
            development_weight: clamp(
                (self.development_weight as f32 + normal.sample(rng) as f32) as i32,
                1,
                100,
            ),
            mobility_weight: clamp(
                (self.mobility_weight as f32 + normal.sample(rng) as f32) as i32,
                1,
                100,
            ),
        }
    }
}

/// Engine variant with performance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineVariant {
    pub id: u32,
    pub params: HeuristicParams,
    pub win_rate: f32,          // Fraction of games won (0.0-1.0)
    pub draw_count: u32,        // Number of draws
    pub loss_count: u32,        // Number of losses
    pub games_played: u32,
    pub fitness_score: f32,     // Composite score (used for selection)
}

impl EngineVariant {
    pub fn new(id: u32, params: HeuristicParams) -> Self {
        Self {
            id,
            params,
            win_rate: 0.0,
            draw_count: 0,
            loss_count: 0,
            games_played: 0,
            fitness_score: 0.0,
        }
    }

    /// Update statistics after game result
    /// Returns (self_result, opponent_result)
    /// Results: 1.0 = win, 0.5 = draw, 0.0 = loss
    pub fn record_game_result(&mut self, result: f32) {
        self.games_played += 1;
        if result == 1.0 {
            self.win_rate = (self.win_rate * (self.games_played - 1) as f32 + 1.0)
                / self.games_played as f32;
        } else if result == 0.5 {
            self.draw_count += 1;
            self.win_rate = (self.win_rate * (self.games_played - 1) as f32 + 0.5)
                / self.games_played as f32;
        } else {
            self.loss_count += 1;
            // win_rate decreases
            self.win_rate = (self.win_rate * (self.games_played - 1) as f32)
                / self.games_played as f32;
        }
    }

    /// Calculate composite fitness (win rate with small bonus for draws)
    pub fn calculate_fitness(&mut self) {
        if self.games_played == 0 {
            self.fitness_score = 0.0;
        } else {
            // Fitness = win_rate + 0.1 * (draw_count / games_played)
            // Encourages wins while valuing draws
            let draw_bonus = (self.draw_count as f32 / self.games_played as f32) * 0.1;
            self.fitness_score = self.win_rate + draw_bonus;
        }
    }

    /// Create a mutated copy with new ID
    pub fn mutate(&self, new_id: u32, std_dev: f32, rng: &mut impl Rng) -> Self {
        let mutated_params = self.params.mutate(std_dev, rng);
        Self::new(new_id, mutated_params)
    }
}

/// Helper function to clamp value
fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
