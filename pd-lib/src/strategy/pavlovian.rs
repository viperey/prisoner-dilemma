use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyPavlovian};

impl Strategy for StrategyPavlovian {
    fn decide(&self, history: &History) -> Move {
        history.last_round()
            .map(|last_round| {
                if last_round.my_move == last_round.their_move {
                    Move::Cooperate
                } else {
                    Move::Deflect
                }
            })
            .unwrap_or(Move::Cooperate)
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::Pavlovian
    }

    fn description(&self) -> String {
        r#"
        Pavlov (or Win-stay, Lose-shift). Cooperates if it and its opponent moved alike in previous
        move and defects if they moved differently.
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
    fn pavlovian_cooperates_first() {
        let strategy = StrategyPavlovian;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn pavlovian_cooperates_on_same_move() {
        let strategy = StrategyPavlovian;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn pavlovian_deflects_on_different_move() {
        let strategy = StrategyPavlovian;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }
}