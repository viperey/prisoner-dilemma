use crate::domain::Prisoner;
use crate::game::GameHandler;
use crate::game_result::GameResult;
use rayon::prelude::*;

pub struct MatchSettings {
    pub num_games: i32,
    pub num_rounds: i32,
}

pub struct MatchResult {
    pub game_results: Vec<GameResult>,
    pub prisoner_a_score: usize,
    pub prisoner_b_score: usize,
}

impl MatchResult {
    pub fn new(game_results: Vec<GameResult>) -> MatchResult {
        let scores = Self::get_scores(&game_results);
        MatchResult {
            game_results,
            prisoner_a_score: scores.0,
            prisoner_b_score: scores.1,
        }
    }

    fn get_scores(game_results: &[GameResult]) -> (usize, usize) {
        game_results.iter().fold((0, 0), |(a, b), game_result| {
            let (partial_count_a, partial_count_b) = game_result.get_score();
            (partial_count_a + a, partial_count_b + b)
        })
    }
}

pub struct MatchHandler;
impl MatchHandler {
    pub fn play(
        match_settings: &MatchSettings,
        prisoner_a: &Prisoner,
        prisoner_b: &Prisoner,
    ) -> MatchResult {
        let game_results: Vec<GameResult> = (0..match_settings.num_games)
            .into_par_iter()
            .map(|_| GameHandler::play(match_settings.num_rounds, prisoner_a, prisoner_b))
            .collect();
        MatchResult::new(game_results)
    }
}
