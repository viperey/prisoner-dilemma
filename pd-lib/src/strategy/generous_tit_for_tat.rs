use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{StrategyTrait};
use rand::Rng;

pub struct StrategyGenerousTitForTat;
impl StrategyTrait for StrategyGenerousTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            Move::Cooperate
        } else {
            history
                .last_round()
                .map(|last_round| last_round.their_move())
                .unwrap_or(Move::Cooperate)
        }
    }
}
