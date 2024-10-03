use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyTitForTat};

impl Strategy for StrategyTitForTat {
    fn decide(&self, history: &History) -> Move {
        history.last_round()
            .map(|last_round| last_round.their_move.clone())
            .unwrap_or(Move::Cooperate)
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::TitForTat
    }

    fn description(&self) -> String {
        "Start by cooperating, then copy the other player\'s moves.".to_string()
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
    fn tit_for_tat_cooperates_first() {
        let strategy = StrategyTitForTat;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_tat_mimics_opponent_cooperate() {
        let strategy = StrategyTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_tat_mimics_opponent_deflect() {
        let strategy = StrategyTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn tit_for_tat_continues_mimicry() {
        let strategy = StrategyTitForTat;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}
