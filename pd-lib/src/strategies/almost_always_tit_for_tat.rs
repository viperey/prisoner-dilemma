use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::round::Round;
use crate::strategies::StrategyBehavior;
use rand::Rng;

pub struct StrategyAlmostAlwaysTitForTat;
impl StrategyBehavior for StrategyAlmostAlwaysTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        let mut rng = rand::thread_rng();
        let is_tit_for_tat: bool = rng.gen_bool(0.9);
        let maybe_their_last_move = history.last_round().map(Round::their_move);
        match (is_tit_for_tat, maybe_their_last_move) {
            (true, Some(their_last_move)) => their_last_move,
            (false, Some(Move::Cooperate)) => Move::Defect,
            (false, Some(Move::Defect)) => Move::Cooperate,
            (_, None) => Move::Cooperate,
        }
    }
}
