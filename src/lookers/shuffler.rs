use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::core::{Game, Position};

use super::looker::Looker;

pub struct Shuffler(Box<dyn Looker>, ThreadRng);

impl Shuffler {
    pub fn new(looker: Box<dyn Looker>, thread_rng: ThreadRng) -> Self {
        Self(looker, thread_rng)
    }
}

impl Looker for Shuffler {
    fn moves(&mut self, game: &Game) -> Vec<Position> {
        let slice: &mut [Position] = &mut self.0.moves(game);
        slice.shuffle(&mut self.1);
        slice.to_vec()
    }
}
