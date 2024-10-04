use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{ StrategyTrait};

pub struct StrategyAlwaysCooperate;
impl StrategyTrait for StrategyAlwaysCooperate {
    fn decide(_: &PartialGameResult) -> Move {
        Move::Cooperate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn always_cooperate_with_empty_history() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyAlwaysCooperate::decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_cooperate_history() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyAlwaysCooperate::decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_defect_history() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyAlwaysCooperate::decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_mixed_history() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyAlwaysCooperate::decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_consistency_check() {
        let history = PartialGameResult::new();
        for _ in 0..1000 {
            assert_eq!(StrategyAlwaysCooperate::decide(&history), Move::Cooperate);
        }
    }
}
