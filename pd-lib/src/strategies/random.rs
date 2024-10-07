use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;
use rand::Rng;

pub struct StrategyRandom;
impl StrategyBehavior for StrategyRandom {
    fn decide(_history: &PartialGameResult) -> Move {
        let mut rng = rand::thread_rng();
        let is_cooperate: bool = rng.gen_bool(0.5);
        match is_cooperate {
            true => Move::Cooperate,
            false => Move::Defect,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Move;
    use crate::game_result::PartialGameResult;

    #[test]
    fn random_strategy_probability_check() {
        let history = PartialGameResult::new();
        let mut cooperate_count = 0;
        let mut defect_count = 0;
        let trials = 1000;

        for _ in 0..trials {
            match StrategyRandom::decide(&history) {
                Move::Cooperate => cooperate_count += 1,
                Move::Defect => defect_count += 1,
            }
        }

        let cooperate_rate = (cooperate_count as f64) / (trials as f64);
        let defect_rate = (defect_count as f64) / (trials as f64);
        let expected_rate = 0.5;
        let error_margin = 0.1;

        assert!(
            (cooperate_rate - expected_rate).abs() <= error_margin,
            "Cooperation rate of {:.3} not within acceptable range of {:.3} ± {:.3}",
            cooperate_rate,
            expected_rate,
            error_margin
        );

        assert!(
            (defect_rate - expected_rate).abs() <= error_margin,
            "Defect rate of {:.3} not within acceptable range of {:.3} ± {:.3}",
            defect_rate,
            expected_rate,
            error_margin
        );
    }
}
