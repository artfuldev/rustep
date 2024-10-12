use std::sync::Arc;

use anyhow::{bail, Result};
use num::BigUint;
use rand::{seq::SliceRandom, thread_rng};

use crate::core::{moves, Game, Move, Position, Time};

use super::Player;

#[derive(Clone)]
pub struct Evaluator(Arc<dyn Fn(Game) -> i64 + Send + Sync + 'static>);

impl Evaluator {
    pub fn new(evaluate: impl Fn(Game) -> i64 + Send + Sync + 'static) -> Self {
        Self(Arc::new(evaluate))
    }
}

impl Player for Evaluator {
    fn best(self, game: Game, _: Option<Time>) -> Result<Position> {
        if game.board.playable.clone() == BigUint::ZERO {
            bail!("No moves left!");
        }
        let mut score = i64::MIN;
        let sign = match game.x_to_play {
            true => 1i64,
            false => -1i64,
        };
        let mut best_moves: Vec<Move> = vec![];
        for mov in moves(game.clone()) {
            let made = game.clone().make(mov.clone());
            let evaluation = sign * self.0(made);
            if evaluation < score {
                continue;
            }
            if evaluation == score {
                best_moves.push(mov);
                continue;
            }
            best_moves.clear();
            score = evaluation;
            best_moves.push(mov);
        }
        let best = best_moves.choose(&mut thread_rng()).expect("impossible");
        Ok(Position::new(best.clone(), game.board.size.into()))
    }
}
