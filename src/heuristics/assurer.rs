use crate::core::{Game, Side};

use super::{assurance::Assurance, Heuristic};

pub struct Assurer(Box<dyn Heuristic>);

impl Assurer {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Self {
        Self(heuristic)
    }
}

impl Heuristic for Assurer {
    fn score(&mut self, game: &Game) -> i64 {
        match Assurance::of(game) {
            Some(Assurance(Side::X, count)) => {
                i64::MAX - (game.moves.len() as i64) - (count as i64)
            }
            Some(Assurance(Side::O, count)) => {
                i64::MIN + (game.moves.len() as i64) + (count as i64)
            }
            None => self.0.score(game),
        }
    }
}
