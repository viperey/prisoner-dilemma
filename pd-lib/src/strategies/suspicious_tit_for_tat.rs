use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategySuspiciousTitForTat;
impl StrategyBehavior for StrategySuspiciousTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| last_round.their_move())
            .unwrap_or(Move::Defect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn tit_for_tat_defect_first() {
        let history = PartialGameResult::new();
        assert_eq!(StrategySuspiciousTitForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn tit_for_tat_mimics_opponent_cooperate() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(
            StrategySuspiciousTitForTat::decide(&history),
            Move::Cooperate
        );
    }

    #[test]
    fn tit_for_tat_mimics_opponent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategySuspiciousTitForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn tit_for_tat_continues_mimicry() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(
            StrategySuspiciousTitForTat::decide(&history),
            Move::Cooperate
        );
    }
}
