use rand::Rng;
use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyRandom};

impl Strategy for StrategyRandom {
    fn decide(&self, _history: &History) -> Move {
        let mut rng = rand::thread_rng();
        let is_cooperate: bool = rng.gen_bool(0.5);
        match is_cooperate {
            true => Move::Cooperate,
            false => Move::Deflect,
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Random
    }

    fn description(&self) -> String {
        "Randomly chooses to cooperate or deflect.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::prisoner::Move;

    #[test]
    fn random_strategy_produces_both_moves() {
        let strategy = StrategyRandom;
        let history = History::new();
        let mut found_cooperate = false;
        let mut found_deflect = false;

        for _ in 0..1000 {
            match strategy.decide(&history) {
                Move::Cooperate => found_cooperate = true,
                Move::Deflect => found_deflect = true,
            }
            if found_cooperate && found_deflect {
                break;
            }
        }
        assert!(found_cooperate, "Strategy didn't produce Cooperate move.");
        assert!(found_deflect, "Strategy didn't produce Deflect move.");
    }
}