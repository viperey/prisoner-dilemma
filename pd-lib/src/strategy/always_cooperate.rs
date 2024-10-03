use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAlwaysCooperate};

impl Strategy for StrategyAlwaysCooperate {
    fn decide(&self, _: &History) -> Move {
        Move::Cooperate
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::AlwaysCooperate
    }

    fn description(&self) -> String {
        "Always cooperate.".to_string()
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
    fn always_cooperate_with_empty_history() {
        let strategy = StrategyAlwaysCooperate;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_cooperate_history() {
        let strategy = StrategyAlwaysCooperate;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_deflect_history() {
        let strategy = StrategyAlwaysCooperate;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_with_mixed_history() {
        let strategy = StrategyAlwaysCooperate;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn always_cooperate_consistency_check() {
        let strategy = StrategyAlwaysCooperate;
        let history = History::new();
        for _ in 0..1000 {
            assert_eq!(strategy.decide(&history), Move::Cooperate);
        }
    }
}