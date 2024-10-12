use crate::{game::Game, position::Position, time::Time};
use anyhow::Result;

pub trait Player {
    fn best(self, game: Game, time: Option<Time>) -> Result<Position>;
}
