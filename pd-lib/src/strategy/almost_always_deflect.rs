use rand::Rng;
use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAlmostAlwaysCooperate, StrategyAlmostAlwaysDeflect};

impl Strategy for StrategyAlmostAlwaysDeflect {
    fn decide(&self, _: &History) -> Move {
        let mut rng = rand::thread_rng();
        let is_cooperate: bool = rng.gen_bool(0.1);
        match is_cooperate {
            true => Move::Cooperate,
            false => Move::Deflect,
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::AlmostAlwaysDefect
    }

    fn description(&self) -> String {
        "Always cooperate, but make a mistake 10% of the time.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;

    #[test]
    fn almost_always_deflect_probability_check() {
        let strategy = StrategyAlmostAlwaysDeflect;
        let history = History::new();
        let mut cooperate_count = 0;
        let trials = 100_000;

        for _ in 0..trials {
            if strategy.decide(&history) == Move::Cooperate {
                cooperate_count += 1;
            }
        }

        let cooperation_rate = (cooperate_count as f64) / (trials as f64);
        let expected_cooperation_rate = 0.1;
        let error_margin = 0.05;

        assert!(
            (cooperation_rate - expected_cooperation_rate).abs() <= error_margin,
            "Cooperation rate of {:.3} not within acceptable range of {:.3} ± {:.3}",
            cooperation_rate,
            expected_cooperation_rate,
            error_margin
        );
    }
}