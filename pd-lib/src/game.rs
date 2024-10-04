use crate::domain::*;
use crate::game_result::{GameResult, PartialGameResult};
use crate::round::Round;
use crate::strategy::StrategyFacade;
use log::debug;

pub struct GameHandler;
impl GameHandler {
    pub fn play(num_rounds: i32, prisoner_a: &Prisoner, prisoner_b: &Prisoner) -> GameResult {
        let game_history: PartialGameResult =
            (0..num_rounds).fold(PartialGameResult::new(), |partial_game_result, _| {
                Self::play_round(&prisoner_a, &prisoner_b, partial_game_result)
            });
        GameResult {
            rounds: game_history.rounds.clone(),
        }
    }

    fn play_round(
        prisoner_a: &Prisoner,
        prisoner_b: &Prisoner,
        mut partial_game_result: PartialGameResult,
    ) -> PartialGameResult {
        let prisoner_a_game_results: &PartialGameResult = &partial_game_result.as_prisoner_a();
        let prisoner_b_game_results: &PartialGameResult = &partial_game_result.as_prisoner_b();
        let prisoner_a_move = StrategyFacade::decide(&prisoner_a, prisoner_a_game_results);
        let prisoner_b_move = StrategyFacade::decide(&prisoner_b, prisoner_b_game_results);
        debug!(
            "Round {}: Prisoner A ({:#?}) move: {:#?}, Prisoner B ({:#?}) move: {:#?}",
            0, prisoner_a.strategy, prisoner_a_move, prisoner_b.strategy, prisoner_b_move
        );
        partial_game_result.add_round(Round::new(prisoner_a_move, prisoner_b_move));
        partial_game_result
    }
}
