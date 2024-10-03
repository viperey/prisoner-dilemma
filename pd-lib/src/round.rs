use crate::prisoner::Move;

#[derive(Debug, Clone)]
pub struct Round {
    pub my_move: Move,
    pub their_move: Move,
}

impl Round {
    pub fn new(my_move: Move, their_move: Move) -> Round {
        Round {
            my_move,
            their_move,
        }
    }
}