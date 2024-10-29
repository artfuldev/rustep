use crate::core::{Cell, Game, Side};

use super::{
    antis::antis, columns::columns, diagonals::diagonals, line::Line, rows::rows, Heuristic,
};

pub struct Consecutive;
impl Consecutive {
    fn score(&mut self, line: &Line, game: &Game) -> i64 {
        let count = line.len() as u8;
        if count < game.win_length {
            return 0;
        }
        let mut score = 0;
        let mut current_count = 0;
        let mut current_side: Option<Side> = None;

        for position in line.iter() {
            match game.get(position) {
                Some(Cell::Played(side)) => {
                    if Some(side.clone()) == current_side {
                        current_count += 1;
                    } else {
                        score += self.aggregate(current_count, current_side);
                        current_count = 1;
                        current_side = Some(side);
                    }
                }
                _ => {
                    score += self.aggregate(current_count, current_side);
                    current_count = 0;
                    current_side = None;
                }
            }
        }
        score += self.aggregate(current_count, current_side);

        score
    }

    fn aggregate(&self, count: usize, side: Option<Side>) -> i64 {
        match side {
            Some(Side::X) => count as i64,
            Some(Side::O) => -(count as i64),
            None => 0,
        }
    }
}

impl Heuristic for Consecutive {
    fn score(&mut self, game: &Game) -> i64 {
        let mut score = 0;
        for line in rows(game.size) {
            score += self.score(&line, game);
        }
        for line in columns(game.size) {
            score += self.score(&line, game);
        }
        for line in diagonals(game.size) {
            score += self.score(&line, game);
        }
        for line in antis(game.size) {
            score += self.score(&line, game);
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Position;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_consecutive_returns_consecutive_xs() -> Result<()> {
        let (_, game) = Game::parse("3x/3_/3_ o")?;
        let line = vec![Position(0, 0), Position(0, 1), Position(0, 2)];
        let actual = Consecutive.score(&line, &game);
        assert_eq!(actual, 3);
        Ok(())
    }

    #[test]
    fn test_consecutive_returns_consecutive_os() -> Result<()> {
        let (_, game) = Game::parse("3o/3_/3_ x")?;
        let line = vec![Position(0, 0), Position(0, 1), Position(0, 2)];
        let actual = Consecutive.score(&line, &game);
        assert_eq!(actual, -3);
        Ok(())
    }

    #[test]
    fn test_consecutive_returns_zero_for_lower_than_win_length() -> Result<()> {
        let (_, game) = Game::parse("3o/3_/3_ x")?;
        let line = vec![Position(0, 0), Position(0, 1)];
        let actual = Consecutive.score(&line, &game);
        assert_eq!(actual, 0);
        Ok(())
    }
}
