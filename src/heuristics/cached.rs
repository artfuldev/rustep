use nohash_hasher::IntMap;

use crate::{core::Game, hashers::Hasher};

use super::Heuristic;

pub struct Cached(Box<dyn Heuristic>, IntMap<u64, i64>, Box<dyn Hasher>);
impl Cached {
    pub fn new(heuristic: Box<dyn Heuristic>, hasher: Box<dyn Hasher>) -> Self {
        Self(heuristic, IntMap::default(), hasher)
    }
}

impl Heuristic for Cached {
    fn score(&mut self, game: &Game) -> i64 {
        match self.1.get(&game.hash) {
            Some(&score) => score,
            None => {
                let score = self.0.score(game);
                for key in self.2.hashes(game) {
                    self.1.insert(key, score);
                }
                score
            }
        }
    }
}
