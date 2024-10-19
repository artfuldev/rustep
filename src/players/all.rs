use crate::core::{Game, Position};

use super::looker::Looker;

#[derive(Clone)]
pub struct All;

impl Looker for All {
    fn moves(&mut self, game: Game) -> Vec<Position> {
        Vec::from_iter(game.playable.iter().map(|x| x.clone()))
    }
}
