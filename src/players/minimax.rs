use std::{i64, sync::Arc};

use anyhow::{bail, Result};
use num::BigUint;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    core::{moves, Game, Move, Position, Time},
    evaluation::{terminal, Heuristic},
};

use super::Player;

#[derive(Clone)]
pub struct Minimax(Arc<dyn Heuristic + Send + Sync + 'static>);

impl Minimax {
    pub fn new(evaluate: impl Heuristic + Send + Sync + 'static) -> Self {
        Self(Arc::new(evaluate))
    }

    fn score(self, game: Game, depth: u8, maximizing: bool) -> i64 {
        if depth == 0 || terminal(game.clone()) {
            return self.0.score(game);
        }
        if maximizing {
            let mut value = i64::MIN;
            for mov in moves(game.clone()) {
                let made = game.clone().make(mov);
                value = value.max(self.clone().score(made, depth - 1, false));
            }
            return value;
        } else {
            let mut value = i64::MAX;
            for mov in moves(game.clone()) {
                let made = game.clone().make(mov);
                value = value.min(self.clone().score(made, depth - 1, true));
            }
            return value;
        }
    }
}

impl Player for Minimax {
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
            let evaluation = sign * self.clone().score(made, 1, !game.x_to_play);
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

#[cfg(test)]
mod tests {

    use crate::evaluation::Dumb;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_winning_move_for_x() -> Result<()> {
        let player = Minimax::new(Dumb);
        let (_, won) = Game::parse("x2_/_x_/2o_ x")?;
        let best = format!("{}", player.best(won.clone(), None)?);
        assert_eq!(best, "c3");
        Ok(())
    }

    #[test]
    fn test_saving_move_for_x() -> Result<()> {
        let player: Minimax = Minimax::new(Dumb);
        let (_, won) = Game::parse("xox/_o_/3_ x")?;
        let best = format!("{}", player.best(won.clone(), None)?);
        assert_eq!(best, "b3");
        Ok(())
    }

    #[test]
    fn test_saving_move_for_x_with_win_length() -> Result<()> {
        let player = Minimax::new(Dumb);
        let (_, mut won) = Game::parse("2o_x_/5_/2_x2_/5_/5_ x")?;
        won.set_win_length(3);
        let best = format!("{}", player.best(won.clone(), None)?);
        assert_eq!(best, "c1");
        Ok(())
    }
}
