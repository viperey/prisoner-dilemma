use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{StrategyTrait};

pub struct StrategyTitForTat;
impl StrategyTrait for StrategyTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| last_round.their_move())
            .unwrap_or(Move::Cooperate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn tit_for_tat_cooperates_first() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_tat_mimics_opponent_cooperate() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_tat_mimics_opponent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyTitForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn tit_for_tat_continues_mimicry() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyTitForTat::decide(&history), Move::Cooperate);
    }
}
