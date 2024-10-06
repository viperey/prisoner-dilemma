use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::StrategyTrait;

pub struct StrategyTitForTwoTats;
impl StrategyTrait for StrategyTitForTwoTats {
    fn decide(history: &PartialGameResult) -> Move {
        let rounds = &history.rounds;
        let total_rounds = rounds.len();

        if total_rounds < 2 {
            return Move::Cooperate;
        }

        let last_two_moves = &rounds[total_rounds - 2..];
        if last_two_moves
            .iter()
            .all(|round| round.their_move() == Move::Defect)
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
    fn tit_for_two_tats_cooperates_initially() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_less_than_two_rounds() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_no_consecutive_defects() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_defects_after_two_consecutive_defects() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Defect);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_not_both_last_moves_were_defects() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_resumes_cooperating_if_defects_not_consecutive() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyTitForTwoTats::decide(&history), Move::Cooperate);
    }
}
