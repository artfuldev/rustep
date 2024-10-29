use crate::core::{Game, Side};

use super::{termination::Termination, Heuristic};

pub struct Win;
impl Win {}

impl Heuristic for Win {
    fn score(&mut self, game: &Game) -> i64 {
        let cost = game.moves.len() as i64;
        match Termination::of(game) {
            None => 0,
            Some(Termination::Drawn) => 0,
            Some(Termination::Won(side)) => {
                if side == Side::X {
                    i64::MAX - cost
                } else {
                    i64::MIN + cost
                }
            }
        }
    }
}
