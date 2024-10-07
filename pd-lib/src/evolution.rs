use crate::domain::Prisoner;
use crate::generation::GenerationHandler;
use crate::r#match::MatchSettings;
use crate::tournament::{TournamentHandler, TournamentResult};
use either::Either;

pub struct EvolutionSettings {
    pub population_size: i32,
    pub num_generations: i32,
    pub match_settings: MatchSettings,
}

pub struct EvolutionHandler;
impl EvolutionHandler {
    pub fn play(evolution_settings: &EvolutionSettings) {
        let generation_seed = Either::Left(evolution_settings.population_size);
        let mut generation: Vec<Prisoner> = GenerationHandler::generate(generation_seed);
        let match_settings = &evolution_settings.match_settings;
        for i in 0..evolution_settings.num_generations {
            let results = TournamentHandler::play(&generation, match_settings);
            let generation_seed: Either<i32, (TournamentResult, i32)> = Either::Right((results, i));
            generation = GenerationHandler::generate(generation_seed);
        }
    }
}
