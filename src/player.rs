use crate::{game::Game, position::Position};
use anyhow::Result;

pub trait Player {
    fn best(self, game: Game) -> Result<Position>;
}
