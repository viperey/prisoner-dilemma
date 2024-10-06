use crate::domain::Move;
use crate::game_result::PartialGameResult;
use crate::strategy::StrategyTrait;

pub struct StrategyPerDDC;
impl StrategyTrait for StrategyPerDDC {
    fn decide(history: &PartialGameResult) -> Move {
        let round_number = history.rounds.len();
        match round_number % 3 {
            0 | 1 => Move::Defect,
            2 => Move::Cooperate,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_result::PartialGameResult;
    use crate::round::Round;

    fn expected_move(round_number: usize) -> Move {
        match round_number % 3 {
            0 | 1 => Move::Defect,
            2 => Move::Cooperate,
            _ => unreachable!(),
        }
    }

    #[test]
    fn per_ddc_margin_of_error_in_modular_sequence() {
        let mut history = PartialGameResult::new();

        for i in 0..9 {
            assert_eq!(StrategyPerDDC::decide(&history), expected_move(i));
            history.add_round(Round::new(Move::Cooperate, expected_move(i)));
        }
    }
}
