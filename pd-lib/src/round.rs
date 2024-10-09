use crate::domain::Move;

#[derive(Debug, Clone)]
pub struct Round {
    prisoner_a: Move,
    prisoner_b: Move,
}

impl Round {
    pub fn new(my_move: Move, their_move: Move) -> Round {
        Round {
            prisoner_a: my_move,
            prisoner_b: their_move,
        }
    }

    pub fn my_move(&self) -> &Move {
        &self.prisoner_a
    }

    pub fn their_move(&self) -> &Move {
        &self.prisoner_b
    }

    pub fn as_prisoner_a(&self) -> Self {
        Round {
            prisoner_a: self.my_move().to_owned(),
            prisoner_b: self.their_move().to_owned(),
        }
    }

    pub fn as_prisoner_b(&self) -> Self {
        Round {
            prisoner_a: self.their_move().to_owned(),
            prisoner_b: self.my_move().to_owned(),
        }
    }
}
