use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyHardTitForTat;
impl StrategyBehavior for StrategyHardTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        let total_rounds = history.rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &history.rounds[total_rounds.saturating_sub(2)..];
        if last_two_moves
            .iter()
            .any(|round| *round.their_move() == Move::Defect)
        {
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
    fn hard_tit_for_tat_cooperates_first_two_moves() {
        let mut history = PartialGameResult::new();
        assert_eq!(StrategyHardTitForTat::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyHardTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn hard_tit_for_tat_cooperates_after_recent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyHardTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn hard_tit_for_tat_betrays_if_any_of_last_two_moves_was_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyHardTitForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn hard_tit_for_tat_cooperates_if_no_recent_defection() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyHardTitForTat::decide(&history), Move::Cooperate);
    }
}
