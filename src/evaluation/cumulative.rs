use crate::core::Game;

use super::Heuristic;

pub struct Cumulative(pub Vec<Box<dyn Heuristic>>);

impl Heuristic for Cumulative {
    fn score(&mut self, game: &Game) -> i64 {
        self.0.iter_mut().fold(0, |acc, h| acc + h.score(game))
    }
}
