use crate::{domain::Move, game_result::PartialGameResult};

use super::StrategyBehavior;

pub struct StrategyOmegaTitForTat;

impl StrategyBehavior for StrategyOmegaTitForTat {
    fn decide(history: &PartialGameResult) -> Move {
        const DEADLOCK_THRESHOLD: i32 = 3;
        const RANDOMNESS_THRESHOLD: i32 = 8;

        let rounds = &history.rounds;
        let len = rounds.len();

        if len == 0 {
            // Cooperate on the first move
            return Move::Cooperate;
        }

        if len == 1 {
            // Copy opponent's move on the second move
            return rounds[0].their_move();
        }

        let mut randomness_counter = 0;
        let mut deadlock_counter = 0;

        // Recompute counters from history
        for i in 1..len {
            let opp_prev_move = rounds[i - 1].their_move();
            let opp_last_move = rounds[i].their_move();
            let my_last_move = rounds[i - 1].my_move();

            // Update randomness_counter
            if opp_prev_move == Move::Cooperate && opp_last_move == Move::Cooperate {
                randomness_counter -= 1;
            }
            if opp_prev_move != opp_last_move {
                randomness_counter += 1;
            }
            if my_last_move != opp_last_move {
                randomness_counter += 1;
            }

            // Update deadlock_counter
            if opp_prev_move != opp_last_move {
                deadlock_counter += 1;
            } else {
                deadlock_counter = 0;
            }
        }

        // Decide next move based on counters
        if deadlock_counter >= DEADLOCK_THRESHOLD {
            Move::Cooperate
        } else if randomness_counter >= RANDOMNESS_THRESHOLD {
            Move::Defect
        } else {
            // Tit for Tat
            rounds.last().unwrap().their_move()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Move::{Cooperate, Defect};
    use crate::round::Round;

    #[test]
    fn test_initial_move() {
        let history = PartialGameResult { rounds: vec![] };
        let decision = StrategyOmegaTitForTat::decide(&history);
        assert_eq!(decision, Cooperate);
    }

    #[test]
    fn test_second_move() {
        let history = PartialGameResult {
            rounds: vec![Round::new(Cooperate, Defect)],
        };
        let decision = StrategyOmegaTitForTat::decide(&history);
        assert_eq!(decision, Defect);
    }

    #[test]
    fn test_tit_for_tat_behavior() {
        let history = PartialGameResult {
            rounds: vec![
                Round::new(Cooperate, Cooperate),
                Round::new(Cooperate, Cooperate),
            ],
        };
        let decision = StrategyOmegaTitForTat::decide(&history);
        assert_eq!(decision, Cooperate);
    }

    #[test]
    fn test_deadlock_breaking() {
        let history = PartialGameResult {
            rounds: vec![
                Round::new(Cooperate, Defect),
                Round::new(Defect, Cooperate),
                Round::new(Cooperate, Defect),
                Round::new(Defect, Cooperate),
            ],
        };
        let decision = StrategyOmegaTitForTat::decide(&history);
        // Should cooperate to break deadlock
        assert_eq!(decision, Cooperate);
    }

    #[test]
    fn test_omega_tit_for_tat_longer_game_version() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Defect, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Defect, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Cooperate, Cooperate));
        history.add_round(Round::new(Cooperate, Cooperate));
        history.add_round(Round::new(Cooperate, Cooperate));
        history.add_round(Round::new(Cooperate, Cooperate));

        let decision = StrategyOmegaTitForTat::decide(&history);
        assert_eq!(decision, Cooperate);
    }

    #[test]
    fn test_omega_tit_for_tat_longer_game_version_2() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Cooperate, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Defect, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Defect, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Cooperate, Cooperate));
        history.add_round(Round::new(Cooperate, Defect));
        history.add_round(Round::new(Defect, Cooperate));
        history.add_round(Round::new(Defect, Defect));
        history.add_round(Round::new(Defect, Defect));
        history.add_round(Round::new(Defect, Defect));
        history.add_round(Round::new(Defect, Defect));
        history.add_round(Round::new(Defect, Defect));

        let decision = StrategyOmegaTitForTat::decide(&history);
        assert_eq!(decision, Defect);
    }
}
