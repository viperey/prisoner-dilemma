use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategySlowTitForTat};

impl Strategy for StrategySlowTitForTat {
    fn decide(&self, history: &History) -> Move {
        let total_rounds = history.rounds.len();
        if total_rounds < 2 {
            return Move::Cooperate;
        }
        let last_two_moves = &history.rounds[total_rounds.saturating_sub(2)..];
        if last_two_moves.iter().all(|round| round.their_move == Move::Deflect) {
            Move::Deflect
        } else if last_two_moves.iter().all(|round| round.their_move == Move::Cooperate) {
            Move::Cooperate
        } else {
            history.last_round().unwrap().their_move.clone()
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::SlowTitForTat
    }

    fn description(&self) -> String {
        "Cooperates first two moves, then defects after two consecutive opponent defections and cooperates after two consecutive cooperations.".to_string()
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
    fn slow_tit_for_tat_cooperates_first_two_moves() {
        let strategy = StrategySlowTitForTat;
        let mut history = History::new();

        // First move should be Cooperate
        assert_eq!(strategy.decide(&history), Move::Cooperate);

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        // Second move should also be Cooperate
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn slow_tit_for_tat_defects_after_double_defection() {
        let strategy = StrategySlowTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));

        // Should defect after two consecutive opponent defections
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn slow_tit_for_tat_cooperates_after_double_cooperation() {
        let strategy = StrategySlowTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));

        // Should cooperate after two consecutive opponent cooperations
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn slow_tit_for_tat_mimics_last_move() {
        let strategy = StrategySlowTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));

        // Should mimic opponent's last move if not two consecutive same moves
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }
}