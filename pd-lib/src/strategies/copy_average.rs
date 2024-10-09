use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;
use rand::Rng;

pub struct StrategyCopyAverage;
impl StrategyBehavior for StrategyCopyAverage {
    fn decide(history: &PartialGameResult) -> Move {
        let total_rounds = history.rounds.len();
        if total_rounds == 0 {
            return Move::Cooperate;
        }

        let cooperate_count = history
            .rounds
            .iter()
            .filter(|round| *round.their_move() == Move::Cooperate)
            .count();

        let cooperate_prob = cooperate_count as f64 / total_rounds as f64;
        let is_cooperate = rand::thread_rng().gen_bool(cooperate_prob);

        if is_cooperate {
            Move::Cooperate
        } else {
            Move::Defect
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    fn generate_history(percent_cooperate: u32, total_rounds: u32) -> PartialGameResult {
        let mut history = PartialGameResult::new();
        let num_cooperate = (total_rounds * percent_cooperate / 100) as usize;

        for _ in 0..num_cooperate {
            history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        }
        for _ in num_cooperate as u32..total_rounds {
            history.add_round(Round::new(Move::Defect, Move::Defect));
        }

        history
    }

    #[test]
    fn copy_average_respects_probability_distributions() {
        let total_rounds = 1000;

        let cooperation_rates = vec![10, 20, 25, 33, 50, 66, 75, 80, 90, 95, 99];

        for &percent in &cooperation_rates {
            let history = generate_history(percent, total_rounds);

            let mut cooperate_count = 0;
            let trials = 10_000;
            for _ in 0..trials {
                if StrategyCopyAverage::decide(&history) == Move::Cooperate {
                    cooperate_count += 1;
                }
            }

            let actual_cooperation_rate = cooperate_count as f64 / trials as f64 * 100.0;
            let error_margin = 10.0;
            assert!(
                (actual_cooperation_rate - (percent as f64)).abs() <= error_margin,
                "Expected cooperation rate around {}%, but got {:.2}%",
                percent,
                actual_cooperation_rate
            );
        }
    }
}
