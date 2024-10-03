use crate::history::History;
use crate::prisoner::Move;
use crate::strategy::{PrisonerStrategy, Strategy, StrategyTitForTwoTats};

impl Strategy for StrategyTitForTwoTats {
    fn decide(&self, history: &History) -> Move {
        let rounds = &history.rounds;
        let total_rounds = rounds.len();

        if total_rounds < 2 {
            return Move::Cooperate;
        }

        let last_two_moves = &rounds[total_rounds - 2..];
        if last_two_moves.iter().all(|round| round.their_move == Move::Deflect) {
            Move::Deflect
        } else {
            Move::Cooperate
        }
    }

    fn name(&self) -> PrisonerStrategy {
        PrisonerStrategy::TitForTwoTats
    }

    fn description(&self) -> String {
        "Always cooperate, unless the other player has defected the last two times.".to_string()
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
    fn tit_for_two_tats_cooperates_initially() {
        let strategy = StrategyTitForTwoTats;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_less_than_two_rounds() {
        let strategy = StrategyTitForTwoTats;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_no_consecutive_deflects() {
        let strategy = StrategyTitForTwoTats;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_deflects_after_two_consecutive_deflects() {
        let strategy = StrategyTitForTwoTats;
        let mut history = History::new();
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Deflect);
    }

    #[test]
    fn tit_for_two_tats_cooperates_if_not_both_last_moves_were_deflects() {
        let strategy = StrategyTitForTwoTats;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }

    #[test]
    fn tit_for_two_tats_resumes_cooperating_if_deflects_not_consecutive() {
        let strategy = StrategyTitForTwoTats;
        let mut history = History::new();
        history.add_round(Round::new(Move::Deflect, Move::Deflect));
        history.add_round(Round::new(Move::Cooperate, Move::Cooperate));
        history.add_round(Round::new(Move::Cooperate, Move::Deflect));
        assert_eq!(strategy.decide(&history), Move::Cooperate);
    }
}