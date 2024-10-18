use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

use super::Side;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Unplayable,
    Playable,
    Played(Side),
}

impl Cell {
    pub fn parse(input: &str) -> IResult<&str, Cell> {
        alt((
            value(Cell::Unplayable, tag(".")),
            value(Cell::Playable, tag("_")),
            value(Cell::Played(Side::X), tag("x")),
            value(Cell::Played(Side::O), tag("o")),
        ))(input)
    }
}
