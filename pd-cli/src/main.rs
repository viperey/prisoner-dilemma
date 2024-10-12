use pd_lib::{
    evolution::{EvolutionHandler, EvolutionSettings},
    prisoner::PrisonerBuilder,
    r#match::MatchSettings,
};

fn main() {
    env_logger::try_init().ok();
    let population = {
        (0..100)
            .flat_map(|_| PrisonerBuilder::deterministic())
            .collect()
    };
    let evolution_settings = EvolutionSettings {
        population,
        num_generations: 100,
        match_settings: MatchSettings {
            num_games: 10,
            num_rounds: 200,
        },
    };
    EvolutionHandler::play(&evolution_settings);
}
