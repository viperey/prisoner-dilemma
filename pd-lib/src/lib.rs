mod domain;
mod game;
mod game_result;
mod r#match;
mod prisoner;
mod round;
mod strategy;
mod tournament;
mod generation;
mod evolution;
// Generation > Tournament > Match > Game > Round

#[cfg(test)]
mod tests {
    use crate::evolution::{EvolutionHandler, EvolutionSettings};
    use crate::prisoner::{Prisoner, PrisonerBuilder};
    use crate::r#match::MatchSettings;
    use crate::tournament::TournamentHandler;

    // #[test]
    fn test_tournament() {
        env_logger::try_init().ok();
        let match_settings = MatchSettings {
            num_games: 10,
            num_rounds: 200,
        };
        let all_prisoners: Vec<Prisoner> = PrisonerBuilder::_all();
        TournamentHandler::play(&all_prisoners, &match_settings);
    }

    #[test]
    fn test_evolution() {
        env_logger::try_init().ok();
        let evolution_settings = EvolutionSettings {
            population_size: 10,
            num_generations: 10,
            match_settings: MatchSettings {
                num_games: 10,
                num_rounds: 200,
            },
        };
        EvolutionHandler::play(&evolution_settings);
    }
}
