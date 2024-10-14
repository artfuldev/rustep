use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace1, combinator::value,
    sequence::separated_pair, IResult,
};

use super::{Board, Move};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    pub board: Board,
    pub win_length: u8,
    pub x_to_play: bool,
}

fn parse_x_to_play(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("x")), value(false, tag("o"))))(input)
}

impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        let (remaining, (board, x_to_play)) =
            separated_pair(Board::parse, multispace1, parse_x_to_play)(input)?;
        return Ok((
            remaining,
            Game {
                board: board.clone(),
                x_to_play,
                win_length: board.size,
            },
        ));
    }

    pub(crate) fn set_win_length(&mut self, win_length: u8) -> () {
        self.win_length = win_length;
    }

    pub fn make(self, mov: Move) -> Self {
        Self {
            board: self.board.clone().make(mov, self.x_to_play),
            x_to_play: !self.x_to_play,
            ..self.clone()
        }
    }

    pub fn mutable_make(&mut self, mov: Move) {
        self.board.mutable_make(mov.clone(), self.x_to_play);
        self.x_to_play = !self.x_to_play;
    }

    pub fn mutable_unmake(&mut self, mov: Move) {
        self.board.mutable_unmake(mov.clone(), self.x_to_play);
        self.x_to_play = !self.x_to_play;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::moves;

    use super::*;
    use num::BigUint;
    use anyhow::Result;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_make_removes_playable() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_make(mov.clone());
        assert!(!moves(game).contains(&mov));
        Ok(())
    }

    #[test]
    fn test_make_changes_side_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let clone = game.clone();
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_make(mov.clone());
        assert_ne!(game.x_to_play, clone.x_to_play);
        Ok(())
    }

    #[test]
    fn test_make_adds_x_if_x_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_make(mov.clone());
        assert_eq!(game.board.played_x.clone() & mov.clone(), mov.clone());
        Ok(())
    }

    #[test]
    fn test_make_adds_o_if_o_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ o")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_make(mov.clone());
        assert_eq!(game.board.played_o.clone() & mov.clone(), mov.clone());
        Ok(())
    }

    #[test]
    fn test_unmake_readds_playable() -> Result<()> {
        let (_, mut game) = Game::parse("3_/_x_/3_ o")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_unmake(mov.clone());
        assert_eq!(game.board.playable.clone() & mov.clone(), mov.clone());
        Ok(())
    }

    #[test]
    fn test_unmake_changes_side_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/_x_/3_ o")?;
        let clone = game.clone();
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_unmake(mov.clone());
        assert_ne!(game.x_to_play, clone.x_to_play);
        Ok(())
    }

    #[test]
    fn test_unmake_removes_o_if_x_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/xo_/3_ x")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_unmake(mov.clone());
        assert_ne!(game.board.played_o.clone() & mov.clone(), mov.clone());
        Ok(())
    }

    #[test]
    fn test_unmake_removes_x_if_o_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/_x_/3_ o")?;
        let mov = BigUint::from(1u8) << 4u8;
        game.mutable_unmake(mov.clone());
        assert_ne!(game.board.played_x.clone() & mov.clone(), mov.clone());
        Ok(())
    }

    #[test]
    fn test_make_unmake_results_in_original() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let clone = game.clone();
        let mov = BigUint::from(1u8);
        game.mutable_make(mov.clone());
        game.mutable_unmake(mov.clone());
        assert_eq!(clone, game);
        Ok(())
    }

}
