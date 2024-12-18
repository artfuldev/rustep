use anyhow::{bail, Result};
use rand::{rngs::ThreadRng, Rng};

use crate::{
    core::{Game, Position, Time},
    lookers::Looker,
};

use super::Player;

pub struct Random(pub Box<dyn Looker>, pub ThreadRng);

impl Player for Random {
    fn best(&mut self, game: &mut Game, _: Option<Time>) -> Result<Position> {
        let moves = self.0.moves(&game);
        let count = moves.len();
        if count == 0 {
            bail!("No moves left!");
        }
        Ok(moves[self.1.gen_range(0..count)].clone())
    }
}
