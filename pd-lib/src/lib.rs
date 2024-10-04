mod domain;
mod game;
mod game_result;
mod r#match;
mod prisoner;
mod round;
mod strategy;
// Tournament > Match > Game > Round

#[cfg(test)]
mod tests {
    use crate::domain::{Prisoner, StrategyName};
    use crate::prisoner::PrisonerBuilder;
    use crate::r#match::{MatchHandler, MatchResult, MatchSettings};
    use log::debug;
    use std::collections::HashMap;

    #[test]
    fn test_match() {
        env_logger::init();
        let match_settings = MatchSettings {
            num_games: 10,
            num_rounds: 200,
        };
        let all_prisoners: Vec<Prisoner> = vec![
            PrisonerBuilder::almost_always_cooperate(),
            PrisonerBuilder::almost_always_defect(),
            PrisonerBuilder::always_alternate(),
            PrisonerBuilder::always_cooperate(),
            PrisonerBuilder::always_defect(),
            PrisonerBuilder::appease(),
            PrisonerBuilder::copy_average(),
            PrisonerBuilder::generous_tit_for_tat(),
            PrisonerBuilder::gradual(),
            PrisonerBuilder::grim_trigger(),
            PrisonerBuilder::hard_majo(),
            PrisonerBuilder::hard_tit_for_tat(),
            PrisonerBuilder::mistrust(),
            PrisonerBuilder::pavlovian(),
            PrisonerBuilder::per_ccb(),
            PrisonerBuilder::per_ddc(),
            PrisonerBuilder::prober(),
            PrisonerBuilder::random(),
            PrisonerBuilder::slow_tit_for_tat(),
            PrisonerBuilder::soft_majo(),
            PrisonerBuilder::tit_for_two_tats(),
            PrisonerBuilder::tit_for_tat(),
            PrisonerBuilder::two_tits_for_tat(),
        ];

        let mut tournament_scores: HashMap<StrategyName, usize> = HashMap::new();

        for prisoner in &all_prisoners {
            let x: StrategyName = prisoner.strategy.clone();
            tournament_scores.insert(x, 0);
        }

        for (i, prisoner_a) in all_prisoners.iter().enumerate() {
            for prisoner_b in all_prisoners.iter().skip(i) {
                let match_result: MatchResult =
                    MatchHandler::play(&match_settings, prisoner_a, prisoner_b);
                let (points_a, points_b) = MatchHandler::get_score(&match_result);
                debug!(
                    "Match result: {:#?}={:#?} vs {:#?}={:#?}",
                    prisoner_a, points_a, prisoner_b, points_b
                );
                *tournament_scores.get_mut(&prisoner_a.strategy).unwrap() += points_a;
                *tournament_scores.get_mut(&prisoner_b.strategy).unwrap() += points_b;
            }
        }

        let mut ranking: Vec<_> = tournament_scores.iter().collect();
        ranking.sort_by(|a, b| b.1.cmp(a.1));

        debug!("Strategy Rankings:");
        for (strategy, score) in ranking {
            debug!("{:#?}: {} points", strategy, score);
            println!("{:#?}: {} points", strategy, score);
        }
    }
}
