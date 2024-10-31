use crate::{
    core::{Game, Position, Side, Time},
    heuristics::Heuristic,
    lookers::Looker,
};
use anyhow::{bail, Result};

use super::Player;

fn terminal(_: &mut Game) -> bool {
    false
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
        if depth == 0 || terminal(game) {
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
        heuristics::{Chance, Win},
        lookers::Nearby,
    };

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_thinker_returns_move() -> Result<()> {
        let (_, mut game) = Game::parse(
            "15_/14_o/15_/_o13_/15_/3_o3_x7_/15_/5_x_x7_/7_x7_/7_x7_/15_/8_x6_/15_/o6_2o6_/15_ x",
        )?;
        game.set_win_length(5);
        let mut thinker = Thinker::with_depth(
            Box::new(Win::new(Box::new(Chance))),
            Box::new(Nearby::new(2)),
            1,
        );
        let position = thinker.best(&mut game, None)?;
        assert_eq!(position, Position(6, 7));
        Ok(())
    }
}
