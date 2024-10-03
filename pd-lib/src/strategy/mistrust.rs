use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyMistrust};

impl Strategy for StrategyMistrust {
    fn decide(&self, history: &History) -> Move {
        if history.rounds.is_empty() {
            Move::Deflect
        } else {
            history.last_round()
                .map(|last_round| last_round.their_move.clone())
                .unwrap_or(Move::Deflect)
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Mistrust
    }

    fn description(&self) -> String {
        "Defects initially, then mimics the opponent's last move.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;

    #[test]
    fn mistrust_defects_first() {
        let strategy = StrategyMistrust;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn mistrust_mimics_opponent_cooperate() {
        let strategy = StrategyMistrust;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn mistrust_mimics_opponent_betray() {
        let strategy = StrategyMistrust;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn mistrust_continues_mimicry() {
        let strategy = StrategyMistrust;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}