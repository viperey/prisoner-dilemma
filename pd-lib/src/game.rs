use crate::history::History;
use crate::prisoner::{Move, Prisoner};
use crate::round::Round;

pub struct Game {
    pub history: History,
    prisoner_a: Prisoner,
    prisoner_b: Prisoner,
}

impl Game {
    pub fn new(prisoner_a: Prisoner, prisoner_b: Prisoner) -> Game {
        Game {
            history: History::new(),
            prisoner_a,
            prisoner_b,
        }
    }

    pub fn play(&mut self, num_rounds: i32) {
        for _ in 0..num_rounds {
            self.play_round();
        }
    }

    pub fn play_round(&mut self) {
        let prisoner_a_move = self.prisoner_a.decide(&self.history.as_prisoner_a());
        let prisoner_b_move = self.prisoner_b.decide(&self.history.as_prisoner_b());
        self.add_round_result(&prisoner_a_move, &prisoner_b_move);
        // println!(
        //     "Round {}: Prisoner A ({:#?}) move: {:#?}, Prisoner B ({:#?}) move: {:#?}",
        //     self.history.rounds.len(),
        //     self.prisoner_a.strategy.name(),
        //     prisoner_a_move,
        //     self.prisoner_b.strategy.name(),
        //     prisoner_b_move
        // );
    }

    pub fn add_round_result(&mut self, prisoner_a_move: &Move, prisoner_b_move: &Move) {
        let round = Round::new(prisoner_a_move.clone(), prisoner_b_move.clone());
        self.history.add_round(round);
    }
}
