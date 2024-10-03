use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAlwaysDefect};

impl Strategy for StrategyAlwaysDefect {
    fn decide(&self, _: &History) -> Move {
        Move::Deflect
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::AlwaysDefect
    }

    fn description(&self) -> String {
        "Always deflect.".to_string()
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
    fn always_defect_with_empty_history() {
        let strategy = StrategyAlwaysDefect;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn always_defect_with_cooperate_history() {
        let strategy = StrategyAlwaysDefect;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn always_defect_with_deflect_history() {
        let strategy = StrategyAlwaysDefect;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn always_defect_with_mixed_history() {
        let strategy = StrategyAlwaysDefect;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn always_defect_consistency_check() {
        let strategy = StrategyAlwaysDefect;
        let history = History::new();

        for _ in 0..1000 {
            assert_eq!(strategy.decide(&history), Move::Deflect);
        }
    }
}