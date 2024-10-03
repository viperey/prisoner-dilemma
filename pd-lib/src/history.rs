use crate::prisoner::Move;
use crate::round::Round;

pub struct History {
    pub rounds: Vec<Round>,
}

impl History {
    pub fn new() -> History {
        History { rounds: Vec::new() }
    }

    pub fn last_round(&self) -> Option<&Round> {
        self.rounds.last()
    }

    pub fn add_round(&mut self, round: Round) {
        self.rounds.push(round);
    }

    pub fn as_prisoner_a(&self) -> Self {
        History {
            rounds: self.rounds.clone(),
        }
    }

    pub fn as_prisoner_b(&self) -> Self {
        History {
            rounds: self
                .rounds
                .clone()
                .into_iter()
                .map(|round| Round::new(round.their_move.clone(), round.my_move.clone()))
                .collect(),
        }
    }
    pub fn get_prisoner_a_points(&self) -> usize {
        self.rounds.iter()
            .map(|round| Self::get_round_points(&round.my_move, &round.their_move))
            .sum()
    }

    pub fn get_prisoner_b_points(&self) -> usize {
        self.rounds.iter()
            .map(|round| Self::get_round_points(&round.their_move, &round.my_move))
            .sum()
    }

    fn get_round_points(my_move: &Move, their_move: &Move) -> usize {
        match (my_move, their_move) {
            (Move::Cooperate, Move::Deflect) => 0,
            (Move::Cooperate, Move::Cooperate) => 3,
            (Move::Deflect, Move::Cooperate) => 5,
            (Move::Deflect, Move::Deflect) => 1,
        }
    }
}
