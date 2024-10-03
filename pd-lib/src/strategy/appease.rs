use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyAppease};

impl Strategy for StrategyAppease {
    fn decide(&self, history: &History) -> Move {
        history.last_round()
            .map(|last_round| {
                return if last_round.their_move == Move::Cooperate {
                    last_round.clone().my_move
                } else if last_round.my_move == Move::Cooperate {
                    Move::Deflect
                }else {
                    Move::Cooperate
                }
            })
            .unwrap_or(Move::Cooperate)
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Appease
    }

    fn description(&self) -> String {
        r#"
        Start by cooperating, then repeat your previous move if the other player has cooperated or
        do the opposite if they have defected.
        "#.to_string()
    }

    fn nicesness_score(&self) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::round::Round;

    #[test]
    fn appease_cooperates_first() {
        let strategy = StrategyAppease;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn appease_repeats_on_opponent_cooperate() {
        let strategy = StrategyAppease;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn appease_opposites_on_opponent_deflect() {
        let strategy = StrategyAppease;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn appease_continues_with_opponent_behavior() {
        let strategy = StrategyAppease;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}