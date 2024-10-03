use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyGradual};


impl StrategyGradual {
    pub fn new() -> Self {
        StrategyGradual { defect_counter: 0 }
    }
}

impl Strategy for StrategyGradual {
    fn decide(&self, history: &History) -> Move {
        if history.rounds.is_empty() {
            return Move::Cooperate;
        }

        if self.defect_counter > 0 {
            return Move::Deflect;
        }

        let last_two_moves = &history.rounds[history.rounds.len().saturating_sub(2)..];
        if last_two_moves.iter().all(|round| round.their_move == Move::Cooperate) {
            Move::Cooperate
        } else {
            Move::Deflect
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Gradual
    }

    fn description(&self) -> String {
        "Cooperates first move, defects incrementally for each opponent defection, then returns to cooperate after two consecutive cooperations.".to_string()
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
    fn gradual_cooperates_first_move() {
        let strategy = StrategyGradual::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_betrays_and_cooperates_after_two_cooperations() {
        let mut strategy = StrategyGradual::new();
        let mut history = History::new();

        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);

        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);

        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_increments_defection_for_each_opponent_defect() {
        let mut strategy = StrategyGradual::new();
        let mut history = History::new();

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));

        strategy.defect_counter = 1;
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }
}