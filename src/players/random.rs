use anyhow::{bail, Result};

use crate::core::{Game, Position, Time};

use super::Player;

#[derive(Clone)]
pub struct Random;

impl Player for Random {
    fn best(self, game: Game, _: Option<Time>) -> Result<Position> {
        if game.playable.len() == 0 {
            bail!("No moves left!");
        }
        Ok(game.playable.iter().next().unwrap().clone())
    }
}
