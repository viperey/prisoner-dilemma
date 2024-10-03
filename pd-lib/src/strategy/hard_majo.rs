use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyHardMajo};

impl Strategy for StrategyHardMajo {
    fn decide(&self, history: &History) -> Move {
        let cooperations = history.rounds.iter().filter(|round| round.their_move == Move::Cooperate).count();
        let defections = history.rounds.iter().filter(|round| round.their_move == Move::Deflect).count();

        if defections >= cooperations {
            Move::Deflect
        } else {
            Move::Cooperate
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::HardMajo
    }

    fn description(&self) -> String {
        "Defects initially and defects if the opponent’s defections are greater or equal to cooperations.".to_string()
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
    fn hard_majo_defects_initially() {
        let strategy = StrategyHardMajo;
        let history = History::new();

        // Should defect initially with no rounds played
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn hard_majo_defects_when_equal_cooperations_and_defections() {
        let strategy = StrategyHardMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Deflect));

        // Should defect because defections are equal to cooperations
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn hard_majo_defects_when_more_defections() {
        let strategy = StrategyHardMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));

        // Should defect because defections are greater than cooperations
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn hard_majo_cooperates_when_more_cooperations() {
        let strategy = StrategyHardMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));

        // Should cooperate because cooperations are greater than defections
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}