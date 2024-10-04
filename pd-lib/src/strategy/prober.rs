use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::{StrategyTrait};

pub struct StrategyProber;
impl StrategyTrait for StrategyProber {
    fn decide(history: &PartialGameResult) -> Move {
        let round_number = history.rounds.len();

        match round_number {
            0 => Move::Defect,
            1 | 2 => Move::Cooperate,
            _ => {
                let initiated_tester_defection = history.rounds[1].their_move() == Move::Cooperate
                    && history.rounds[2].their_move() == Move::Cooperate;

                if initiated_tester_defection {
                    Move::Defect
                } else {
                    history
                        .last_round()
                        .map(|last_round| last_round.their_move())
                        .unwrap_or(Move::Cooperate)
                }
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
    fn prober_starts_with_dcc() {
        let mut history = PartialGameResult::new();
        assert_eq!(StrategyProber::decide(&history), Move::Defect);

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyProber::decide(&history), Move::Cooperate);

        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyProber::decide(&history), Move::Cooperate);
    }

    #[test]
    fn prober_switches_to_always_defect_if_cooperated_first_three() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyProber::decide(&history), Move::Defect);
    }

    #[test]
    fn prober_switches_to_tit_for_tat_if_any_defection_in_first_three() {
        let mut history = PartialGameResult::new();

        history.add_round(Round::new(Move::Defect, Move::Defect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyProber::decide(&history), Move::Defect);

        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyProber::decide(&history), Move::Cooperate);
    }
}
