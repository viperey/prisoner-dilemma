use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyProber};

impl Strategy for StrategyProber {
    fn decide(&self, history: &History) -> Move {
        let round_number = history.rounds.len();

        match round_number {
            0 => Move::Deflect, // First round: Defect (D)
            1 | 2 => Move::Cooperate, // Second and third round: Cooperate (C)
            _ => {
                // Check the opponent's move in the second and third round
                let initiated_tester_defection = history.rounds[1].their_move == Move::Cooperate
                    && history.rounds[2].their_move == Move::Cooperate;

                if initiated_tester_defection {
                    // If opponent cooperated both times, switch to always defect
                    Move::Deflect
                } else {
                    // Otherwise, switch to tit-for-tat
                    history.last_round()
                        .map(|last_round| last_round.their_move.clone())
                        .unwrap_or(Move::Cooperate)
                }
            }
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Prober
    }

    fn description(&self) -> String {
        "Plays D, C, C, then either always defects or plays tit_for_tat based on opponent's second and third responses.".to_string()
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
    fn prober_starts_with_dcc() {
        let strategy = StrategyProber;
        let mut history = History::new();

        assert_eq!(strategy.decide(&history), Move::Deflect);

        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn prober_switches_to_always_defect_if_cooperated_first_three() {
        let strategy = StrategyProber;
        let mut history = History::new();

        // Setup first three moves
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));

        // Fourth move, should continue to betray
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn prober_switches_to_tit_for_tat_if_any_defection_in_first_three() {
        let strategy = StrategyProber;
        let mut history = History::new();

        // Setup first three moves where the opponent defects at least once
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));

        // Fourth move, should mimic last move
        assert_eq!(strategy.decide(&history), Move::Deflect);

        // Add another round for continuity
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}