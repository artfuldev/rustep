use anyhow::{bail, Result};
use num_bigint::BigUint;
use rand::{distributions::Uniform, thread_rng, Rng};

use crate::{game::Game, position::Position};

pub fn best(game: Game) -> Result<Position> {
    if game.playable == BigUint::ZERO {
        bail!("No moves left!");
    }
    let mut rng = thread_rng();
    let square = u16::from(game.size).pow(2);
    let distribution = Uniform::new(0, square);
    let mut sample = rng.sample(distribution);
    let mut position = BigUint::from(1u8) << sample;
    while (position.clone() & game.playable.clone()) != position {
        sample = rng.sample(distribution);
        position = BigUint::from(1u8) << sample;
    }
    let index = square - sample - 1;
    let u16size = u16::from(game.size);
    Ok(Position(index / u16size, index % u16size))
}
