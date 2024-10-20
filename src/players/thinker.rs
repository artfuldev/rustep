use crate::{
    core::{Game, Position, Side, Time},
    evaluation::Heuristic,
    lookers::Looker,
};
use anyhow::{bail, Result};

use super::Player;

fn terminal(_: &mut Game) -> bool {
    false
}

pub struct Thinker(pub Box<dyn Heuristic>, pub Box<dyn Looker>);
impl Thinker {
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
        if depth == 0 || terminal(game) {
            return (best, self.0.score(&game));
        }

        if maximizing {
            let mut value = i64::MIN;
            for position in self.1.moves(&game) {
                game.play(&position);
                let (pv, score) = self.pvs(game, visited + 1, depth - 1, alpha, beta, false);
                game.undo();
                if score > value {
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
            let (pv, score) = self.pvs(game, visited + 1, depth - 1, alpha, beta, true);
            game.undo();
            if score < value {
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
            2 * game.win_length,
            i64::MIN,
            i64::MAX,
            game.side_to_play == Side::X,
        );
        match pv.first() {
            Some(position) => Ok(position.clone()),
            None => bail!("No moves found!"),
        }
    }
}
