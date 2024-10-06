use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::StrategyTrait;

pub struct StrategyAppease;
impl StrategyTrait for StrategyAppease {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                return if last_round.their_move() == Move::Cooperate {
                    last_round.my_move()
                } else if last_round.my_move() == Move::Cooperate {
                    Move::Defect
                } else {
                    Move::Cooperate
                };
            })
            .unwrap_or(Move::Cooperate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    #[test]
    fn appease_cooperates_first() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyAppease::decide(&history), Move::Cooperate);
    }

    #[test]
    fn appease_repeats_on_opponent_cooperate() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyAppease::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyAppease::decide(&history), Move::Defect);
    }

    #[test]
    fn appease_opposites_on_opponent_defect() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyAppease::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyAppease::decide(&history), Move::Cooperate);
    }

    #[test]
    fn appease_continues_with_opponent_behavior() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyAppease::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyAppease::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyAppease::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyAppease::decide(&history), Move::Cooperate);
    }
}
