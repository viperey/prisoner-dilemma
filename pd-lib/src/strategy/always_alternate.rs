use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAlternate};

impl Strategy for StrategyAlternate {
    fn decide(&self, history: &History) -> Move {
        if history.rounds.len() % 2 == 0 {
            Move::Cooperate
        } else {
            Move::Deflect
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Alternate
    }

    fn description(&self) -> String {
        "Alternate between cooperating and deflecting.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;

    #[test]
    fn alternate_cooperates_on_first_round() {
        let strategy = StrategyAlternate;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn alternate_deflects_on_second_round() {
        let strategy = StrategyAlternate;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn alternate_consistent_alternate_pattern() {
        let strategy = StrategyAlternate;
        let mut history = History::new();
        const TOTAL_ROUNDS: usize = 10;

        for i in 0..TOTAL_ROUNDS {
            let expected_move = if i % 2 == 0 {
                Move::Cooperate
            } else {
                Move::Deflect
            };

            assert_eq!(strategy.decide(&history), expected_move);
            history.add_round(Round::new(expected_move.clone(), Move::Cooperate));
        }
    }
}