use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyMistrust;
impl StrategyBehavior for StrategyMistrust {
    fn decide(history: &PartialGameResult) -> Move {
        if history.rounds.is_empty() {
            Move::Defect
        } else {
            history
                .last_round()
                .map(|last_round| last_round.their_move())
                .unwrap_or(Move::Defect)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn mistrust_defects_first() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyMistrust::decide(&history), Move::Defect);
    }

    #[test]
    fn mistrust_mimics_opponent_cooperate() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyMistrust::decide(&history), Move::Cooperate);
    }

    #[test]
    fn mistrust_mimics_opponent_betray() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyMistrust::decide(&history), Move::Defect);
    }

    #[test]
    fn mistrust_continues_mimicry() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyMistrust::decide(&history), Move::Cooperate);
    }
}
