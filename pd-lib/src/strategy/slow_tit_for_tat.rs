use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{ StrategyTrait};

pub struct StrategySlowTitForTat;
impl StrategyTrait for StrategySlowTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        let total_rounds = history.rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &history.rounds[total_rounds.saturating_sub(2)..];
        if last_two_moves
            .iter()
            .all(|round| round.their_move() == Move::Defect)
        {
            Move::Defect
        } else if last_two_moves
            .iter()
            .all(|round| round.their_move() == Move::Cooperate)
        {
            Move::Cooperate
        } else {
            history.last_round().unwrap().their_move()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn slow_tit_for_tat_cooperates_first_two_moves() {
        let mut history = PartialGameResult::new();

        assert_eq!(StrategySlowTitForTat::decide(&history), Move::Cooperate);

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategySlowTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn slow_tit_for_tat_defects_after_double_defection() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategySlowTitForTat::decide(&history), Move::Defect);
    }

    #[test]
    fn slow_tit_for_tat_cooperates_after_double_cooperation() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategySlowTitForTat::decide(&history), Move::Cooperate);
    }

    #[test]
    fn slow_tit_for_tat_mimics_last_move() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategySlowTitForTat::decide(&history), Move::Defect);
    }
}
