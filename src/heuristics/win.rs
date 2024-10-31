use crate::core::{Game, Side};

use super::{termination::Termination, Heuristic};

pub struct Win(Box<dyn Heuristic>);

impl Win {
    pub fn new(heuristic: Box<dyn Heuristic>) -> Self {
        Win(heuristic)
    }
}

impl Heuristic for Win {
    fn score(&mut self, game: &Game) -> i64 {
        match Termination::of(game) {
            Some(Termination::Drawn) => 0,
            Some(Termination::Won(Side::X)) => i64::MAX - (game.moves.len() as i64),
            Some(Termination::Won(Side::O)) => i64::MIN + (game.moves.len() as i64),
            None => self.0.score(game),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::heuristics::Null;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_win_scores_premature_using_heuristic() -> Result<()> {
        let (_, mut game) = Game::parse(
            "15_/14_o/15_/_o13_/15_/3_o3_x7_/15_/5_x_x7_/7_x7_/7_x7_/15_/8_x6_/15_/o6_2o6_/15_ x",
        )?;
        game.set_win_length(5);
        let mut heuristic = Win::new(Box::new(Null));
        assert_eq!(heuristic.score(&game), 0);
        Ok(())
    }
}
