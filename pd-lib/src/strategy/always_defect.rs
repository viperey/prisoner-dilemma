use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{ StrategyTrait};

pub struct StrategyAlwaysDefect;
impl StrategyTrait for StrategyAlwaysDefect {
    fn decide(_: &PartialGameResult) -> Move {
        Move::Defect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn always_defect_with_empty_history() {
        let partial_game_result = PartialGameResult::new();
        assert_eq!(
            StrategyAlwaysDefect::decide(&partial_game_result),
            Move::Defect
        );
    }

    #[test]
    fn always_defect_with_cooperate_history() {
        let mut partial_game_result = PartialGameResult::new();
        partial_game_result.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        partial_game_result.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(
            StrategyAlwaysDefect::decide(&partial_game_result),
            Move::Defect
        );
    }

    #[test]
    fn always_defect_with_defect_history() {
        let mut partial_game_result = PartialGameResult::new();
        partial_game_result.add_round(Round::new(Move::Defect, Move::Defect));
        partial_game_result.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(
            StrategyAlwaysDefect::decide(&partial_game_result),
            Move::Defect
        );
    }

    #[test]
    fn always_defect_with_mixed_history() {
        let mut partial_game_result = PartialGameResult::new();
        partial_game_result.add_round(Round::new(Move::Cooperate, Move::Defect));
        partial_game_result.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(
            StrategyAlwaysDefect::decide(&partial_game_result),
            Move::Defect
        );
    }

    #[test]
    fn always_defect_consistency_check() {
        let history = PartialGameResult::new();

        for _ in 0..1000 {
            assert_eq!(StrategyAlwaysDefect::decide(&history), Move::Defect);
        }
    }
}
