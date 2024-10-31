use super::Hasher;

pub struct Id;

impl Hasher for Id {
    fn hashes(&mut self, game: &crate::core::Game) -> Vec<u64> {
        vec![game.hash]
    }
}
