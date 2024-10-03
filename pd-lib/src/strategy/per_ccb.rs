use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyPerCCD};

impl Strategy for StrategyPerCCD {
    fn decide(&self, history: &History) -> Move {
        let round_number = history.rounds.len();
        match round_number % 3 {
            0 | 1 => Move::Cooperate,
            2 => Move::Deflect,
            _ => unreachable!(),
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::PerDDC
    }

    fn description(&self) -> String {
        "Plays a periodic sequence of D, D, C.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;
    use crate::strategy::StrategyPerCCD;

    fn expected_move(round_number: usize) -> Move {
        match round_number % 3 {
            0 | 1 => Move::Cooperate,
            2 => Move::Deflect,
            _ => unreachable!(),
        }
    }

    #[test]
    fn per_ddc_margin_of_error_in_modular_sequence() {
        let strategy = StrategyPerCCD;
        let mut history = History::new();

        for i in 0..9 {
            assert_eq!(strategy.decide(&history), expected_move(i));
            history.add_round(Round::new(Move::Cooperate, expected_move(i)));
        }
    }
}