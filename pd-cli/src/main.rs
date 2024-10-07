use pd_lib::{
    evolution::{EvolutionHandler, EvolutionSettings},
    r#match::MatchSettings,
};

fn main() {
    env_logger::try_init().ok();
    let evolution_settings = EvolutionSettings {
        population_size: 100,
        num_generations: 10,
        match_settings: MatchSettings {
            num_games: 10,
            num_rounds: 200,
        },
    };
    EvolutionHandler::play(&evolution_settings);
}
