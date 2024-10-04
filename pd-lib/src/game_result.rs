use crate::domain::Move;
use crate::round::Round;

#[derive(Debug, Clone)]
pub struct PartialGameResult {
    pub rounds: Vec<Round>,
}

impl PartialGameResult {
    pub fn new() -> PartialGameResult {
        PartialGameResult { rounds: Vec::new() }
    }

    pub fn last_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn as_prisoner_a(&self) -> Self {
        PartialGameResult {
            rounds: self.rounds.iter().map(Round::as_prisoner_a).collect(),
        }
    }

    pub fn as_prisoner_b(&self) -> Self {
        PartialGameResult {
            rounds: self.rounds.iter().map(Round::as_prisoner_b).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameResult {
    pub rounds: Vec<Round>,
}

impl GameResult {
    pub fn get_score(&self) -> (usize, usize) {
        (self.get_prisoner_a_points(), self.get_prisoner_b_points())
    }

    fn get_prisoner_a_points(&self) -> usize {
        self.rounds
            .iter()
            .map(|round| Self::get_round_points(&round.my_move(), &round.their_move()))
            .sum()
    }

    fn get_prisoner_b_points(&self) -> usize {
        self.rounds
            .iter()
            .map(|round| Self::get_round_points(&round.their_move(), &round.my_move()))
            .sum()
    }

    fn get_round_points(my_move: &Move, their_move: &Move) -> usize {
        match (my_move, their_move) {
            (Move::Cooperate, Move::Defect) => 0,
            (Move::Cooperate, Move::Cooperate) => 3,
            (Move::Defect, Move::Cooperate) => 5,
            (Move::Defect, Move::Defect) => 1,
        }
    }
}
