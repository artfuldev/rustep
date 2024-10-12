use num_bigint::BigUint;

use crate::core::Game;

use super::wins;

#[derive(PartialEq, Debug)]
pub enum Termination {
    Won(bool),
    Drawn,
}

pub fn terminated(game: Game) -> Option<Termination> {
    for win in wins(game.size.into(), game.win_length.into()) {
        if (game.played_x.clone() & win.clone()) == win {
            return Some(Termination::Won(true));
        }
        if (game.played_o.clone() & win.clone()) == win {
            return Some(Termination::Won(false));
        }
    }
    if game.playable == BigUint::ZERO {
        return Some(Termination::Drawn);
    }
    return None;
}

pub fn terminal(game: Game) -> bool {
    terminated(game).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_x_wins() -> Result<()> {
        let (_, game) = Game::parse("x_x/_xo/x_o x")?;
        assert_eq!(terminated(game), Some(Termination::Won(true)));
        Ok(())
    }

    #[test]
    fn test_o_wins() -> Result<()> {
        let (_, game) = Game::parse("x_o/_xo/x_o x")?;
        assert_eq!(terminated(game), Some(Termination::Won(false)));
        Ok(())
    }

    #[test]
    fn test_draw() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/oxo x")?;
        assert_eq!(terminated(game), Some(Termination::Drawn));
        Ok(())
    }

    #[test]
    fn test_no_termination() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/o_o x")?;
        assert_eq!(terminated(game), None);
        Ok(())
    }

    #[test]
    fn test_terminal_x_wins() -> Result<()> {
        let (_, game) = Game::parse("x_x/_xo/x_o x")?;
        assert!(terminal(game));
        Ok(())
    }

    #[test]
    fn test_terminal_o_wins() -> Result<()> {
        let (_, game) = Game::parse("x_o/_xo/x_o x")?;
        assert!(terminal(game));
        Ok(())
    }

    #[test]
    fn test_terminal_draw() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/oxo x")?;
        assert!(terminal(game));
        Ok(())
    }

    #[test]
    fn test_not_terminal() -> Result<()> {
        let (_, game) = Game::parse("xox/xox/o_o x")?;
        assert!(!terminal(game));
        Ok(())
    }
}
