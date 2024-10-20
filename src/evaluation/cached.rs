use rustc_hash::FxHashMap;

use crate::core::Game;

use super::Heuristic;

pub struct Cached(Box<dyn Heuristic>, FxHashMap<u64, i64>);
impl Cached {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Self {
        Self(heuristic, FxHashMap::default())
    }
}

impl Heuristic for Cached {
    fn score(&mut self, game: &Game) -> i64 {
        match self.1.get(&game.hash) {
            Some(&score) => {
                score
            }
            None => {
                let score = self.0.score(game);
                self.1.insert(game.hash, score);
                score
            }
        }
    }
}
