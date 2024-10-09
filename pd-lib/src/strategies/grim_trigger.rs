use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyGrimTrigger;
impl StrategyBehavior for StrategyGrimTrigger {
    fn decide(history: &PartialGameResult) -> Move {
        let opponent_defected = history
            .rounds
            .iter()
            .any(|round| *round.their_move() == Move::Defect);
        if opponent_defected {
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
    fn grim_trigger_cooperates_initially() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyGrimTrigger::decide(&history), Move::Cooperate);
    }

    #[test]
    fn grim_trigger_cooperates_without_defection() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGrimTrigger::decide(&history), Move::Cooperate);
    }

    #[test]
    fn grim_trigger_defects_after_first_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGrimTrigger::decide(&history), Move::Defect);
    }

    #[test]
    fn grim_trigger_continues_to_defect_after_defection() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGrimTrigger::decide(&history), Move::Defect);
    }

    #[test]
    fn grim_trigger_handles_no_defections_after_cooperation() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGrimTrigger::decide(&history), Move::Cooperate);
    }
}
