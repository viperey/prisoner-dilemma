use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

use super::utils;

pub struct StrategyExtortionA;
pub struct StrategyExtortionB;
pub struct StrategyExtortionC;
pub struct StrategyExtortionD;
pub struct StrategyExtortionE;
pub struct StrategyExtortionF;

impl StrategyBehavior for StrategyExtortionA {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((8 / 9) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide((2 / 9) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide((11 / 18) as f64),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyExtortionB {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide(0.8),
                    (Move::Cooperate, Move::Defect) => utils::decide(0.1),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.6),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyExtortionC {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((11 / 12) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide((5 / 24) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide((2 / 3) as f64),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyExtortionD {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((5 / 6) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide(0.2),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.5),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyExtortionE {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((17 / 20) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide((3 / 40) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.7),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyExtortionF {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((11 / 15) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide((2 / 15) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide((7 / 15) as f64),
                    (Move::Defect, Move::Defect) => Move::Defect,
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}
