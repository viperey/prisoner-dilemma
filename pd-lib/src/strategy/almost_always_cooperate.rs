use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{StrategyTrait};
use rand::Rng;

pub struct StrategyAlmostAlwaysCooperate;
impl StrategyTrait for StrategyAlmostAlwaysCooperate {
    fn decide(_: &PartialGameResult) -> Move {
        let mut rng = rand::thread_rng();
        let is_cooperate: bool = rng.gen_bool(0.9);
        match is_cooperate {
            true => Move::Cooperate,
            false => Move::Defect,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;

    #[test]
    fn almost_always_cooperate_probability_check() {
        let history = PartialGameResult::new();
        let mut cooperate_count = 0;
        let trials = 100_000;

        for _ in 0..trials {
            if StrategyAlmostAlwaysCooperate::decide(&history) == Move::Cooperate {
                cooperate_count += 1;
            }
        }

        let cooperation_rate = (cooperate_count as f64) / (trials as f64);
        let expected_cooperation_rate = 0.9;
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
