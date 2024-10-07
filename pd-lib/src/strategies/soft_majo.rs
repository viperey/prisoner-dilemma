use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategySoftMajo;
impl StrategyBehavior for StrategySoftMajo {
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

        if cooperations >= defections {
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
    fn soft_majo_cooperates_initially() {
        let history = PartialGameResult::new();
        assert_eq!(StrategySoftMajo::decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_cooperates_when_equal_cooperations_and_defections() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategySoftMajo::decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_cooperates_when_more_cooperations() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategySoftMajo::decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_defects_when_more_defections() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategySoftMajo::decide(&history), Move::Defect);
    }
}
