use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::StrategyTrait;

pub struct StrategyPavlovian;
impl StrategyTrait for StrategyPavlovian {
    fn decide(history: &PartialGameResult) -> Move {
        history
            .last_round()
            .map(|last_round| {
                if last_round.my_move() == last_round.their_move() {
                    Move::Cooperate
                } else {
                    Move::Defect
                }
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
    fn pavlovian_cooperates_first() {
        let history = PartialGameResult::new();
        assert_eq!(StrategyPavlovian::decide(&history), Move::Cooperate);
    }

    #[test]
    fn pavlovian_cooperates_on_same_move() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(StrategyPavlovian::decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Defect, Move::Defect));
        assert_eq!(StrategyPavlovian::decide(&history), Move::Cooperate);
    }

    #[test]
    fn pavlovian_defects_on_different_move() {
        let mut history = PartialGameResult::new();
        history.add_round(Round::new(Move::Cooperate, Move::Defect));
        assert_eq!(StrategyPavlovian::decide(&history), Move::Defect);
        history.add_round(Round::new(Move::Defect, Move::Cooperate));
        assert_eq!(StrategyPavlovian::decide(&history), Move::Defect);
    }
}
