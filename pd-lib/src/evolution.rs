use crate::domain::Prisoner;
use crate::generation::GenerationHandler;
use crate::r#match::MatchSettings;
use crate::tournament::{TournamentHandler, TournamentResult};

pub struct EvolutionSettings {
    pub population: Vec<Prisoner>,
    pub num_generations: i32,
    pub match_settings: MatchSettings,
}

pub struct EvolutionHandler;
impl EvolutionHandler {
    pub fn play(evolution_settings: &EvolutionSettings) {
        let mut population: Vec<Prisoner> = evolution_settings.population.clone();
        let match_settings = &evolution_settings.match_settings;
        for i in 0..evolution_settings.num_generations {
            let results: TournamentResult = TournamentHandler::play(&population, match_settings);
            population = GenerationHandler::generate(results, i);
        }
    }
}
