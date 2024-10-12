use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace1, combinator::value,
    sequence::separated_pair, IResult,
};

use super::{Board, Move};

#[derive(Clone, Debug)]
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
}
