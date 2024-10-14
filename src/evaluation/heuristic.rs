use crate::core::Game;

pub trait Heuristic {
    fn score(self: &Self, game: Game) -> i64;
}
