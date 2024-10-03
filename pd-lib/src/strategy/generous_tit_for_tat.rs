use rand::Rng;
use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyGenerousTitForTat};

impl Strategy for StrategyGenerousTitForTat {
    fn decide(&self, history: &History) -> Move {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            Move::Cooperate
        } else {
            history.last_round()
                .map(|last_round| last_round.their_move.clone())
                .unwrap_or(Move::Cooperate)
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::TitForTat
    }

    fn description(&self) -> String {
        "Start by cooperating, then copy the other player\'s moves.".to_string()
    }

    fn nicesness_score(&self) -> f64 {
        1.0
    }
}
