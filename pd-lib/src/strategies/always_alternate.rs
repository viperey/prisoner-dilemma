use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyAlternate;
impl StrategyBehavior for StrategyAlternate {
    fn decide(history: &PartialGameResult) -> Move {
        if history.rounds.len() % 2 == 0 {
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

    #[test]
    fn alternate_cooperates_on_first_round() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyAlternate::decide(&history), Move::Cooperate);
    }

    #[test]
    fn alternate_defects_on_second_round() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyAlternate::decide(&history), Move::Defect);
    }

    #[test]
    fn alternate_consistent_alternate_pattern() {
        let mut history = PartialGameResult::new();
        const TOTAL_ROUNDS: usize = 10;

        for i in 0..TOTAL_ROUNDS {
            let expected_move = if i % 2 == 0 {
                Move::Cooperate
            } else {
                Move::Defect
            };

            assert_eq!(StrategyAlternate::decide(&history), expected_move);
            history.add_round(Round::new(expected_move.clone(), Move::Cooperate));
        }
    }
}
