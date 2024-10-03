mod strategy;
mod history;
mod game;
mod round;
mod prisoner;

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::prisoner::Prisoner;
    use crate::strategy::PrisonerStrategy;
    use std::collections::HashMap;

    #[test]
    fn test_tournament() {
        // Tournament > Match > Game > Round
        let strategies = vec![
            PrisonerStrategy::TitForTat,
            PrisonerStrategy::AlwaysCooperate,
            PrisonerStrategy::AlwaysDefect,
            PrisonerStrategy::Alternate,
            PrisonerStrategy::Appease,
            PrisonerStrategy::Random,
            PrisonerStrategy::CopyAverage,
            PrisonerStrategy::Pavlovian,
            PrisonerStrategy::TitForTwoTats,
            PrisonerStrategy::TwoTitsForTat,
            PrisonerStrategy::GrimTrigger,
            PrisonerStrategy::SoftMajo,
            PrisonerStrategy::HardMajo,
            PrisonerStrategy::PerDDC,
            PrisonerStrategy::PerCCB,
            PrisonerStrategy::Mistrust,
            PrisonerStrategy::HardTitForTat,
            PrisonerStrategy::SlowTitForTat,
            PrisonerStrategy::Gradual,
            PrisonerStrategy::Prober,
            PrisonerStrategy::AlmostAlwaysCooperate,
            PrisonerStrategy::AlmostAlwaysDefect,
            PrisonerStrategy::GenerousTitForTat,
        ];

        let num_games_per_match = 5;
        let num_rounds_per_game = 200;

        let mut tournament_scores: HashMap<&PrisonerStrategy, usize> = HashMap::new();

        for strategy in &strategies {
            tournament_scores.insert(&strategy, 0);
        }

       for (i, strategy_a) in strategies.iter().enumerate() {
           for strategy_b in strategies.iter().skip(i ) {
               // println!("Matching {:#?} vs {:#?}", &strategy_a, &strategy_b);
                for _ in 0..num_games_per_match {
                    let mut game = Game::new(
                        Prisoner::new(strategy_a.clone()),
                        Prisoner::new(strategy_b.clone()),
                    );

                    game.play(num_rounds_per_game);

                    let points_a = game.history.get_prisoner_a_points();
                    let points_b = game.history.get_prisoner_b_points();
                    // println!("Match result: {:?}={:?} vs {:?}={:?}", strategy_a, points_a, strategy_b, points_b);

                    *tournament_scores.get_mut(strategy_a).unwrap() += points_a;
                    *tournament_scores.get_mut(strategy_b).unwrap() += points_b;
                }
            }
        }

        let mut ranking: Vec<_> = tournament_scores.iter().collect();
        ranking.sort_by(|a, b| b.1.cmp(a.1));

        println!("Strategy Rankings:");
        for (strategy, score) in ranking {
            println!("{:?}: {} points", strategy, score);
        }
    }
}
