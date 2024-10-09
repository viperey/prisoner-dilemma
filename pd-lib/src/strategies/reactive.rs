use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

use super::utils;

pub struct StrategyReactive;
impl StrategyBehavior for StrategyReactive {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| last_round.their_move())
            .map(|their_move| match their_move {
                Move::Cooperate => utils::decide(0.7),
                Move::Defect => utils::decide(0.3),
            })
            .unwrap_or(utils::decide(0.5))
    }
}
