use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategySoftMajo};

impl Strategy for StrategySoftMajo {
    fn decide(&self, history: &History) -> Move {
        let cooperations = history.rounds.iter().filter(|round| round.their_move == Move::Cooperate).count();
        let defections = history.rounds.iter().filter(|round| round.their_move == Move::Deflect).count();

        if cooperations >= defections {
            Move::Cooperate
        } else {
            Move::Deflect
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::SoftMajo
    }

    fn description(&self) -> String {
        "Cooperates as long as the opponent's cooperations are greater than or equal to defections.".to_string()
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
    fn soft_majo_cooperates_initially() {
        let strategy = StrategySoftMajo;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_cooperates_when_equal_cooperations_and_defections() {
        let strategy = StrategySoftMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_cooperates_when_more_cooperations() {
        let strategy = StrategySoftMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn soft_majo_deflects_when_more_defections() {
        let strategy = StrategySoftMajo;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }
}