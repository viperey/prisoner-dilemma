use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{StrategyTrait, };

pub struct StrategyTwoTitsForTat;
impl StrategyTrait for StrategyTwoTitsForTat {
    fn decide(history: &PartialGameResult) -> Move {
        let rounds = &history.rounds;
        let total_rounds = rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &rounds[total_rounds - 2..];
        if last_two_moves
            .iter()
            .any(|round| round.their_move() == Move::Defect)
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
    fn two_tits_for_tat_cooperates_first_two_rounds() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Cooperate);

        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn two_tits_for_tat_cooperates_if_no_recent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn two_tits_for_tat_defects_if_recent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn two_tits_for_tat_detects_defections_across_two_rounds() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn two_tits_for_tat_reverts_to_cooperate_after_non_defections() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTwoTitsForTat::decide(&history), Move::Cooperate);
    }
}
