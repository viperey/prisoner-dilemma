use crate::domain::{Prisoner, StrategyId};
use crate::prisoner::PrisonerBuilder;
use crate::tournament::TournamentResult;
use either::*;
use log::{debug, info};
use std::collections::HashMap;

pub struct GenerationHandler;
impl GenerationHandler {
    pub fn generate(generation_seed: Either<i32, (TournamentResult, i32)>) -> Vec<Prisoner> {
        match generation_seed {
            Left(population_size) => Self::generate_population_for_size(population_size),
            Right((tournament_result, num_generation)) => {
                Self::generate_population_for_tournament(tournament_result, num_generation)
            }
        }
    }

    fn generate_population_for_size(population_size: i32) -> Vec<Prisoner> {
        (0..population_size)
            .flat_map(|_| PrisonerBuilder::_all())
            .collect()
    }

    fn generate_population_for_tournament(
        tournament_result: TournamentResult,
        num_generation: i32,
    ) -> Vec<Prisoner> {
        let tournament_total_points: usize = tournament_result.tournament_scores.values().sum();
        let tournament_total_population: usize = tournament_result.tournament_scores.len();

        let mut strategy_groups: HashMap<StrategyId, Vec<(&Prisoner, &usize)>> = HashMap::new();
        for (prisoner, score) in &tournament_result.tournament_scores {
            strategy_groups
                .entry(prisoner.strategy.id.clone())
                .or_default()
                .push((prisoner, score));
        }

        let next_generation: Vec<Prisoner> = strategy_groups
            .into_iter()
            .flat_map(|(strategy, grouped_scores)| {
                let strategy_scores: i32 = Self::get_strategy_scores(&grouped_scores);
                let score_ratio: f64 = (strategy_scores as f64) / (tournament_total_points as f64);
                let next_generation_amount: usize =
                    (tournament_total_population as f64 * score_ratio) as usize;

                debug!(
                    "Strategy {:#?} with population {} with a probability of {:.3} for survival",
                    strategy,
                    grouped_scores.len(),
                    score_ratio
                );
                (0..next_generation_amount)
                    .map(|_| PrisonerBuilder::from(grouped_scores[0].0.clone()))
                    .collect::<Vec<Prisoner>>()
            })
            .collect();

        Self::print_summary(num_generation, tournament_result, &next_generation);
        next_generation
    }

    fn get_strategy_scores(grouped_scores: &[(&Prisoner, &usize)]) -> i32 {
        grouped_scores.iter().map(|(_, score)| **score as i32).sum()
    }

    fn print_summary(num_generation: i32, result: TournamentResult, next_generation: &[Prisoner]) {
        let current_population = result.get_population_type_count();

        info!(
            "Generation #{}. {} surviving strategies",
            num_generation + 1,
            current_population.len()
        );
        for (strategy, &previous_population_count) in current_population.iter() {
            let new_population_count: usize = next_generation
                .iter()
                .filter(|prisoner| prisoner.strategy.id == *strategy)
                .count();

            let delta = new_population_count as isize - previous_population_count as isize;

            info!(
                "{:#?}: population={}, delta={}",
                strategy, new_population_count, delta
            );
        }
    }
}
