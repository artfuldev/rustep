use anyhow::Result;

use crate::core::{Game, Position, Time};

pub trait Player {
    fn best(self, game: Game, time: Option<Time>) -> Result<Position>;
}
