use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

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
            map(Side::parse, Cell::Played),
        ))(input)
    }
}
