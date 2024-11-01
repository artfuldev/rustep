use crate::{
    core::{Game, Position, Side, Time},
    heuristics::{termination::Termination, Assurance, Heuristic},
    lookers::Looker,
};
use anyhow::{bail, Result};

use super::Player;

#[inline(always)]
fn is_terminal(game: &Game) -> bool {
    Termination::of(game).is_some() || Assurance::of(game).is_some()
}

pub struct Thinker(Box<dyn Heuristic>, Box<dyn Looker>, u8);

impl Thinker {
    pub fn new(heuristic: Box<dyn Heuristic>, looker: Box<dyn Looker>) -> Self {
        Self(heuristic, looker, 2)
    }

    pub fn with_depth(heuristic: Box<dyn Heuristic>, looker: Box<dyn Looker>, depth: u8) -> Self {
        Self(heuristic, looker, depth)
    }

    pub fn pvs(
        &mut self,
        game: &mut Game,
        visited: usize,
        depth: u8,
        mut alpha: i64,
        mut beta: i64,
        maximizing: bool,
    ) -> (Vec<Position>, i64) {
        let mut best = game.moves[visited..].to_vec();
        if depth == 0 || is_terminal(game) {
            return (best, self.0.score(&game));
        }

        if maximizing {
            let mut value = i64::MIN;
            for position in self.1.moves(&game) {
                game.play(&position);
                let (mut pv, score) = self.pvs(game, visited + 1, depth - 1, alpha, beta, false);
                game.undo();
                if score > value {
                    pv.insert(0, position.clone());
                    best = pv;
                    value = score;
                }
                alpha = alpha.max(value);
                if alpha >= beta {
                    break; // Beta cut-off
                }
            }
            return (best, value);
        }

        let mut value = i64::MAX;
        for position in self.1.moves(&game) {
            game.play(&position);
            let (mut pv, score) = self.pvs(game, visited + 1, depth - 1, alpha, beta, true);
            game.undo();
            if score < value {
                pv.insert(0, position.clone());
                best = pv;
                value = score;
            }
            beta = beta.min(value);
            if beta <= alpha {
                break; // Alpha cut-off
            }
        }
        (best, value)
    }
}

impl Player for Thinker {
    fn best(&mut self, game: &mut Game, _: Option<Time>) -> Result<Position> {
        let (pv, _) = self.pvs(
            game,
            game.moves.len(),
            self.2,
            i64::MIN,
            i64::MAX,
            game.side_to_play == Side::X,
        );
        println!("info pv {:?}", pv);
        match pv.first() {
            Some(position) => Ok(position.clone()),
            None => bail!("No moves found!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        heuristics::{Assurer, Chance, Win},
        lookers::{Nearby, Shuffler},
    };

    use super::*;
    use anyhow::Result;
    use rand::thread_rng;

    #[test]
    fn test_thinker_returns_move() -> Result<()> {
        let (_, mut game) = Game::parse(
            "_x8_o4_/_o2x3_o7_/_x_x2_xo_x5_/4_o3_2o5_/_oxo2x_o3_o3_/_2x2o_2x7_/_x2_o2_2x2o_xo_/2_o2_o_2x3_2o_/2_x2_ox3o2x3_/5_xox_x2_x2_/2_2x_o_o_x2_o2_/4_ox2_ox_ox2_/5_x_x3_o3_/5_xo_x3_2o_/6_o_x2_o_o_ x",
        )?;
        game.set_win_length(5);
        let mut thinker = Thinker::with_depth(
            Box::new(Win::new(Box::new(Assurer::new(Box::new(Chance))))),
            Box::new(Shuffler::new(Box::new(Nearby::new(2)), thread_rng())),
            2,
        );
        let position = thinker.best(&mut game, None)?;
        let expected = vec![Position(5, 9), Position(10, 4)];
        assert!(expected.contains(&position));
        Ok(())
    }
}
