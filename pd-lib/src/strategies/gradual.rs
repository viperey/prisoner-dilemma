use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategies::StrategyBehavior;

pub struct StrategyGradual;
impl StrategyBehavior for StrategyGradual {
    fn decide(history: &PartialGameResult) -> Move {
        let mut punishment_limit: u32 = 0u32;
        let mut punishment_count: u32 = 0u32;
        let mut calming: u32 = 0u32;
        let mut punishing: bool = false;

        if history.rounds.is_empty() {
            return Move::Cooperate;
        }

        let rounds = &history.rounds;

        for i in 0..rounds.len() {
            if calming > 0 {
                calming -= 1;
            } else if punishing {
                if punishment_count < punishment_limit {
                    punishment_count += 1;
                } else {
                    punishing = false;
                    punishment_count = 0;
                    calming = 2;
                }
            } else {
                let opponent_last_move = if i == 0 {
                    Move::Cooperate
                } else {
                    rounds[i - 1].their_move()
                };

                if opponent_last_move == Move::Defect {
                    punishing = true;
                    punishment_limit += 1;
                    punishment_count = 1;
                }
            }
        }

        if calming > 0 {
            Move::Cooperate
        } else if punishing {
            if punishment_count < punishment_limit {
                Move::Defect
            } else {
                Move::Cooperate
            }
        } else {
            let opponent_last_move = rounds.last().unwrap().their_move();
            if opponent_last_move == Move::Defect {
                Move::Defect
            } else {
                Move::Cooperate
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn gradual_handles_multiple_defections_and_increments_punishment() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase starts (punishment_limit = 1)
        // Round 3: Strategy defects (punishment_count = 1), punishment complete
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Calming phase (2 rounds of cooperation)
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 2)
        // Round 7: Strategy defects (punishment_count = 1)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Round 8: Strategy defects (punishment_count = 2), punishment complete
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Calming phase (2 rounds of cooperation)
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 3)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Strategy defects (punishment_count = 2)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Strategy defects (punishment_count = 3), punishment complete
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Calming phase (2 rounds of cooperation)
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Verify that after punishment and calming, strategy returns to cooperation
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_resets_punishment_count_after_calming() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Punishment phase (punishment_limit = 2)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Punishment complete, start calming
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Ensure that punishment_count resets
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 3)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_handles_opponent_defecting_during_punishment() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 1)
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Even though opponent defected during punishment, punishment_limit does not increase
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Since opponent defected during punishment, strategy should not increase punishment_limit
        // Opponent defects again
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Punishment phase (punishment_limit = 2)
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
    }

    #[test]
    fn gradual_opponent_always_defects() {
        let mut history = PartialGameResult::new();
        // Simulate opponent always defecting over multiple rounds
        for _ in 0..10 {
            history.add_round(Round::new(StrategyGradual::decide(&history), Move::Defect));
        }
        // The strategy should have increased its punishment_limit accordingly
        // We can check that the strategy is in punishing or calming state
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
    }

    #[test]
    fn gradual_opponent_defects_multiple_times_between_punishments() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 1)
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Round 4: Calming phase, opponent defects
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Round 5: Calming phase, opponent defects
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Cooperate);
        // Round 6: Opponent defects again
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        // Since we are out of calming phase, strategy should increase punishment_limit
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
        // Punishment phase (punishment_limit = 2)
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyGradual::decide(&history), Move::Defect);
    }
}
