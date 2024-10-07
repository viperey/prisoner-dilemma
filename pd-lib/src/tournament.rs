use crate::domain::{Prisoner, StrategyId};
use crate::r#match::{MatchHandler, MatchResult, MatchSettings};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct TournamentResult {
    pub tournament_scores: HashMap<Prisoner, usize>,
}

impl TournamentResult {
    pub fn get_population_type_count(&self) -> HashMap<StrategyId, usize> {
        self.tournament_scores
            .clone()
            .keys()
            .counts_by(|prisoner| prisoner.strategy.id)
    }
}

pub struct TournamentHandler;

impl TournamentHandler {
    pub fn play(population: &[Prisoner], match_settings: &MatchSettings) -> TournamentResult {
        let matches: Vec<(Prisoner, Prisoner)> = Self::build_all_matches(population).to_owned();
        Self::play_matches(match_settings, matches)
    }

    fn play_matches(
        settings: &MatchSettings,
        matches: Vec<(Prisoner, Prisoner)>,
    ) -> TournamentResult {
        let match_results = matches
            .par_iter()
            .map(|(prisoner_a, prisoner_b)| {
                let match_result = Self::play_match(settings, prisoner_a, prisoner_b);
                (
                    (prisoner_a.clone(), match_result.prisoner_a_score),
                    (prisoner_b.clone(), match_result.prisoner_b_score),
                )
            })
            .collect();
        Self::get_scores(match_results)
    }

    fn play_match(
        settings: &MatchSettings,
        prisoner_a: &Prisoner,
        prisoner_b: &Prisoner,
    ) -> MatchResult {
        let match_result = MatchHandler::play(settings, prisoner_a, prisoner_b);
        debug!(
            "Match result: {:#?}={:#?} vs {:#?}={:#?}",
            prisoner_a.strategy.clone(),
            match_result.prisoner_a_score,
            prisoner_b.strategy.clone(),
            match_result.prisoner_b_score,
        );
        match_result
    }

    fn build_all_matches(population: &[Prisoner]) -> Vec<(Prisoner, Prisoner)> {
        population
            .iter()
            .enumerate()
            .flat_map(|(i, prisoner_a)| {
                population
                    .iter()
                    .skip(i)
                    .map(|prisoner_b| (prisoner_a.clone(), prisoner_b.clone()))
            })
            .collect()
    }

    fn get_scores(match_results: Vec<((Prisoner, usize), (Prisoner, usize))>) -> TournamentResult {
        let tournament_scores = match_results.into_iter().fold(
            HashMap::new(),
            |mut acc, ((prisoner_a, points_a), (prisoner_b, points_b))| {
                *acc.entry(prisoner_a.clone()).or_insert(0) += points_a;
                *acc.entry(prisoner_b.clone()).or_insert(0) += points_b;
                acc
            },
        );
        let result = TournamentResult { tournament_scores };
        Self::log_scores(&result);
        result
    }

    fn log_scores(tournament_result: &TournamentResult) {
        let mut ranking: Vec<_> = tournament_result.tournament_scores.iter().collect();
        ranking.sort_by(|a, b| b.1.cmp(a.1));
        debug!("Tournament ranking:");
        for (prisoner, score) in ranking {
            debug!("\t + {:#?}: {} points", prisoner.strategy, score);
        }
    }
}
