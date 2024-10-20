use crate::core::{Game, Position};

pub trait Looker {
    fn moves(&mut self, game: &Game) -> Vec<Position>;
}
