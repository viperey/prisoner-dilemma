use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyGradual;
impl StrategyBehavior for StrategyGradual {
    fn decide(history: &PartialGameResult) -> Move {
        if history.rounds.is_empty() {
            return Move::Cooperate;
        }

        let last_two_moves = &history.rounds[history.rounds.len().saturating_sub(2)..];
        if last_two_moves
            .iter()
            .all(|round| *round.their_move() == Move::Cooperate)
        {
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
    fn gradual_cooperates_first_move() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_betrays_and_cooperates_after_two_cooperations() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_increments_defection_for_each_opponent_defect() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        history.add_round(Round::new(Move::Defect, Move::Cooperate));

        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
    }
}
