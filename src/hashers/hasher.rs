use crate::core::Game;

pub trait Hasher {
    fn hashes(&mut self, game: &Game) -> Vec<u64>;
}
