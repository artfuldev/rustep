use crate::core::{Cell, Game, Side};

use super::wins::wins;

#[derive(Debug, PartialEq, Eq)]
pub struct Assurance(pub Side, pub u8);

impl Assurance {
    pub fn of(game: &Game) -> Option<Assurance> {
        let win_length = game.win_length;
        if win_length >= game.size || win_length < 3 {
            return None;
        }
        match game.moves.last() {
            Some(position) => {
                let winning_lines = wins(position, game.size, win_length + 1);
                let played = game.side_to_play.other();
                for line in winning_lines {
                    let last = line.len() - 1;
                    match (game.get(&line[0]), game.get(&line[last])) {
                        (Some(Cell::Playable), Some(Cell::Playable)) => {
                            let mut count = 0;
                            for pos in &line[1..last] {
                                match game.get(pos) {
                                    Some(Cell::Played(side)) => {
                                        if side == played {
                                            count += 1;
                                        }
                                    }
                                    _ => break,
                                }
                            }
                            if count == win_length - 1 {
                                return Some(Assurance(played, 1));
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                None
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
    fn test_should_call_x_assurance() -> Result<()> {
        let (_, mut game) = Game::parse("5_/5_/_3x_/5_/5_ o")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, Some(Assurance(Side::X, 1)));
        Ok(())
    }

    #[test]
    fn test_should_call_x_assurance_vertical() -> Result<()> {
        let (_, mut game) = Game::parse("5_/4_x/4_x/4_x/5_ o")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, Some(Assurance(Side::X, 1)));
        Ok(())
    }

    #[test]
    fn test_should_call_x_assurance_diagonal() -> Result<()> {
        let (_, mut game) = Game::parse("5_/_x3_/2_x2_/3_x_/5_ o")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, Some(Assurance(Side::X, 1)));
        Ok(())
    }

    #[test]
    fn test_should_call_not_call_premature_x_assurance() -> Result<()> {
        let (_, mut game) = Game::parse("5_/_x3_/2_x2_/5_/5_ o")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, None);
        Ok(())
    }

    #[test]
    fn test_should_call_x_assurance_anti() -> Result<()> {
        let (_, mut game) = Game::parse("5_/3_x_/2_x2_/_x3_/5_ o")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, Some(Assurance(Side::X, 1)));
        Ok(())
    }

    #[test]
    fn test_should_call_o_win() -> Result<()> {
        let (_, mut game) = Game::parse("5_/5_/5_/5_/_3o_ x")?;
        game.set_win_length(4);
        let result = Assurance::of(&game);
        assert_eq!(result, Some(Assurance(Side::O, 1)));
        Ok(())
    }

    #[test]
    fn test_should_not_claim_early_results() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/o_o x")?;
        let result = Assurance::of(&game);
        assert_eq!(result, None);
        Ok(())
    }
}
