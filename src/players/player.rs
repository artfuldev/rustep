use anyhow::Result;

use crate::core::{Game, Position, Time};

pub trait Player {
    fn best(&mut self, game: &mut Game, time: Option<Time>) -> Result<Position>;
}
