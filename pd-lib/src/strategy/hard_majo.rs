use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::StrategyTrait;

pub struct StrategyHardMajo;
impl StrategyTrait for StrategyHardMajo {
    fn decide(history: &PartialGameResult) -> Move {
        let cooperations = history
            .rounds
            .iter()
            .filter(|round| round.their_move() == Move::Cooperate)
            .count();
        let defections = history
            .rounds
            .iter()
            .filter(|round| round.their_move() == Move::Defect)
            .count();

        if defections >= cooperations {
            Move::Defect
        } else {
            Move::Cooperate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn hard_majo_defects_initially() {
        let history = PartialGameResult::new();

        assert_eq!(StrategyHardMajo::decide(&history), Move::Defect);
    }

    #[test]
    fn hard_majo_defects_when_equal_cooperations_and_defections() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyHardMajo::decide(&history), Move::Defect);
    }

    #[test]
    fn hard_majo_defects_when_more_defections() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyHardMajo::decide(&history), Move::Defect);
    }

    #[test]
    fn hard_majo_cooperates_when_more_cooperations() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyHardMajo::decide(&history), Move::Cooperate);
    }
}
