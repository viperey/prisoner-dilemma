use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

use super::utils;

pub struct StrategyEqualizerA;
pub struct StrategyEqualizerB;
pub struct StrategyEqualizerC;
pub struct StrategyEqualizerD;
pub struct StrategyEqualizerE;
pub struct StrategyEqualizerF;

impl StrategyBehavior for StrategyEqualizerA {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: &Move = last_round.my_move();
                let their_move: &Move = last_round.their_move();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide(0.75),
                    (Move::Cooperate, Move::Defect) => utils::decide(0.25),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.5),
                    (Move::Defect, Move::Defect) => utils::decide(0.25),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyEqualizerB {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: &Move = last_round.my_move();
                let their_move: &Move = last_round.their_move();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide(0.9),
                    (Move::Cooperate, Move::Defect) => utils::decide(0.7),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.2),
                    (Move::Defect, Move::Defect) => utils::decide(0.1),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyEqualizerC {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: &Move = last_round.my_move();
                let their_move: &Move = last_round.their_move();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide(0.9),
                    (Move::Cooperate, Move::Defect) => utils::decide(0.5),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.5),
                    (Move::Defect, Move::Defect) => utils::decide(0.3),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyEqualizerD {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((27 / 35) as f64),
                    (Move::Cooperate, Move::Defect) => utils::decide((17 / 35) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.2),
                    (Move::Defect, Move::Defect) => utils::decide((2 / 35) as f64),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyEqualizerE {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => utils::decide((2 / 3) as f64),
                    (Move::Cooperate, Move::Defect) => Move::Defect,
                    (Move::Defect, Move::Cooperate) => utils::decide((2 / 3) as f64),
                    (Move::Defect, Move::Defect) => utils::decide((1 / 3) as f64),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}

impl StrategyBehavior for StrategyEqualizerF {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                let my_move: Move = last_round.my_move().to_owned();
                let their_move: Move = last_round.their_move().to_owned();
                match (my_move, their_move) {
                    (Move::Cooperate, Move::Cooperate) => Move::Cooperate,
                    (Move::Cooperate, Move::Defect) => utils::decide((13 / 15) as f64),
                    (Move::Defect, Move::Cooperate) => utils::decide(0.2),
                    (Move::Defect, Move::Defect) => utils::decide((2 / 5) as f64),
                }
            })
            .unwrap_or(Move::Cooperate)
    }
}
