use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyTwoTitsForTat};

impl Strategy for StrategyTwoTitsForTat {
    fn decide(&self, history: &History) -> Move {
        let rounds = &history.rounds;
        let total_rounds = rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &rounds[total_rounds - 2..];
        if last_two_moves.iter().any(|round| round.their_move == Move::Deflect) {
            Move::Deflect
        } else {
            Move::Cooperate
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::TwoTitsForTat
    }

    fn description(&self) -> String {
        "Always cooperate, unless the other player has deflected at least once in the last two moves.".to_string()
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
    fn two_tits_for_tat_cooperates_first_two_rounds() {
        let strategy = StrategyTwoTitsForTat;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);

        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn two_tits_for_tat_cooperates_if_no_recent_deflect() {
        let strategy = StrategyTwoTitsForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn two_tits_for_tat_deflects_if_recent_deflect() {
        let strategy = StrategyTwoTitsForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn two_tits_for_tat_detects_deflections_across_two_rounds() {
        let strategy = StrategyTwoTitsForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn two_tits_for_tat_reverts_to_cooperate_after_non_deflections() {
        let strategy = StrategyTwoTitsForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}