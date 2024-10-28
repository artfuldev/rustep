use crate::core::{Cell, Game, Side};

use super::wins::wins;

#[derive(Debug, PartialEq, Eq)]
pub enum Termination {
    Drawn,
    Won(Side),
}

impl Termination {
    pub fn of(game: &Game) -> Option<Termination> {
        match game.moves.last() {
            Some(position) => {
                let win_length = game.win_length;
                let winning_lines = wins(position, game.size, win_length);
                let played = game.side_to_play.clone().other();
                for line in winning_lines {
                    let mut count = 0;
                    for pos in &line {
                        match game.get(pos) {
                            Some(Cell::Played(side)) => {
                                if side == played {
                                    count += 1;
                                }
                            }
                            _ => break,
                        }
                    }
                    if count == win_length {
                        return Some(Termination::Won(played));
                    }
                }
                if game.playable.is_empty() {
                    Some(Termination::Drawn)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_should_call_x_win() -> Result<()> {
        let (_, game) = Game::parse("3_/3_/3x o")?;
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::X)));
        Ok(())
    }

    #[test]
    fn test_should_call_x_win_vertical() -> Result<()> {
        let (_, mut game) = Game::parse("4_x/4_x/4_x/5_/5_ o")?;
        game.set_win_length(3);
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::X)));
        Ok(())
    }

    #[test]
    fn test_should_call_x_win_diagonal() -> Result<()> {
        let (_, mut game) = Game::parse("x4_/_x3_/2_x2_/5_/5_ o")?;
        game.set_win_length(3);
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::X)));
        Ok(())
    }

    #[test]
    fn test_should_call_x_win_diagonal_without_win_length() -> Result<()> {
        let (_, game) = Game::parse("x2_/_x_/2_x o")?;
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::X)));
        Ok(())
    }

    #[test]
    fn test_should_call_not_call_premature_x_win() -> Result<()> {
        let (_, game) = Game::parse("x4_/_x3_/2_x2_/5_/5_ o")?;
        let result = Termination::of(&game);
        assert_eq!(result, None);
        Ok(())
    }

    #[test]
    fn test_should_call_x_win_anti() -> Result<()> {
        let (_, mut game) = Game::parse("2_x2_/_x3_/x4_/5_/5_ o")?;
        game.set_win_length(3);
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::X)));
        Ok(())
    }

    #[test]
    fn test_should_call_o_win() -> Result<()> {
        let (_, game) = Game::parse("3_/3_/3o x")?;
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Won(Side::O)));
        Ok(())
    }

    #[test]
    fn test_should_call_draw() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/oxo o")?;
        let result = Termination::of(&game);
        assert_eq!(result, Some(Termination::Drawn));
        Ok(())
    }

    #[test]
    fn test_should_not_claim_early_results() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/o_o x")?;
        let result = Termination::of(&game);
        assert_eq!(result, None);
        Ok(())
    }
}
