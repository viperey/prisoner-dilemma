use crate::domain::Prisoner;
use crate::game::GameHandler;
use crate::game_result::GameResult;

pub struct MatchSettings {
    pub num_games: i32,
    pub num_rounds: i32,
}

pub struct MatchResult {
    pub game_results: Vec<GameResult>,
}

pub struct MatchHandler;
impl MatchHandler {
    pub fn play(
        match_settings: &MatchSettings,
        prisoner_a: &Prisoner,
        prisoner_b: &Prisoner,
    ) -> MatchResult {
        let game_results: Vec<GameResult> = (0..match_settings.num_games)
            .map(|_| GameHandler::play(match_settings.num_rounds, prisoner_a, prisoner_b))
            .collect();
        MatchResult { game_results }
    }

    pub fn get_score(match_result: &MatchResult) -> (usize, usize) {
        match_result
            .game_results
            .iter()
            .fold((0, 0), |(a, b), game_result| {
                let (partial_count_a, partial_count_b) = game_result.get_score();
                (partial_count_a + a, partial_count_b + b)
            })
    }
}
