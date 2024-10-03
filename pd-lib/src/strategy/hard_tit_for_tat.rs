use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyHardTitForTat};

impl Strategy for StrategyHardTitForTat {
    fn decide(&self, history: &History) -> Move {
        let total_rounds = history.rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &history.rounds[total_rounds.saturating_sub(2)..];
        if last_two_moves.iter().any(|round| round.their_move == Move::Deflect) {
            Move::Deflect
        } else {
            Move::Cooperate
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::HardTitForTat
    }

    fn description(&self) -> String {
        "Cooperates first two moves, then defects if the opponent defected in either of the last two moves.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;

    #[test]
    fn hard_tit_for_tat_cooperates_first_two_moves() {
        let strategy = StrategyHardTitForTat;
        let mut history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn hard_tit_for_tat_cooperates_after_recent_defect() {
        let strategy = StrategyHardTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn hard_tit_for_tat_betrays_if_any_of_last_two_moves_was_defect() {
        let strategy = StrategyHardTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn hard_tit_for_tat_cooperates_if_no_recent_defection() {
        let strategy = StrategyHardTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}