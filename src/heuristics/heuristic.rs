use crate::core::Game;

pub trait Heuristic {
    fn score(&mut self, game: &Game) -> i64;
}
