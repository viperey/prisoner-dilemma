use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyGrimTrigger};

impl Strategy for StrategyGrimTrigger {
    fn decide(&self, history: &History) -> Move {
        let opponent_deflected = history.rounds.iter().any(|round| round.their_move == Move::Deflect);
        if opponent_deflected {
            Move::Deflect
        } else {
            Move::Cooperate
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::GrimTrigger
    }

    fn description(&self) -> String {
        "Cooperate until the other player defects, after that always defect".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;

    #[test]
    fn grim_trigger_cooperates_initially() {
        let strategy = StrategyGrimTrigger;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn grim_trigger_cooperates_without_deflection() {
        let strategy = StrategyGrimTrigger;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn grim_trigger_deflects_after_first_defect() {
        let strategy = StrategyGrimTrigger;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn grim_trigger_continues_to_deflect_after_defection() {
        let strategy = StrategyGrimTrigger;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn grim_trigger_handles_no_deflections_after_cooperation() {
        let strategy = StrategyGrimTrigger;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}