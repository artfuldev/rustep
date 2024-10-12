use anyhow::{bail, Result};
use num_bigint::BigUint;
use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

use crate::core::{Game, Position, Time};

use super::Player;

#[derive(Clone)]
pub struct Random(ThreadRng);

impl Random {
    pub fn new(rng: ThreadRng) -> Self {
        Self(rng)
    }
}

impl Player for Random {
    fn best(self, game: Game, _: Option<Time>) -> Result<Position> {
        if game.board.playable == BigUint::ZERO {
            bail!("No moves left!");
        }
        let mut rng = self.0;
        let square = u16::from(game.board.size).pow(2);
        let distribution = Uniform::new(0, square);
        let mut sample = rng.sample(distribution);
        let mut position = BigUint::from(1u8) << sample;
        while (position.clone() & game.board.playable.clone()) != position {
            sample = rng.sample(distribution);
            position = BigUint::from(1u8) << sample;
        }
        Ok(Position::new(position, game.board.size.into()))
    }
}
