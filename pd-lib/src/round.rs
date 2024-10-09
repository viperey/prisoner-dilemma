use std::{rc::Rc, sync::Arc};

use crate::domain::Move;

#[derive(Debug, Clone)]
pub struct Round {
    prisoner_a: Arc<Move>,
    prisoner_b: Arc<Move>,
}

impl Round {
    pub fn new(my_move: Move, their_move: Move) -> Round {
        Round {
            prisoner_a: Arc::new(my_move),
            prisoner_b: Arc::new(their_move),
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
            prisoner_a: self.prisoner_a.clone(),
            prisoner_b: self.prisoner_b.clone(),
        }
    }

    pub fn as_prisoner_b(&self) -> Self {
        Round {
            prisoner_a: self.prisoner_b.clone(),
            prisoner_b: self.prisoner_a.clone(),
        }
    }
}
