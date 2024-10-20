use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    IResult,
};

use super::Side;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Playable,
    Played(Side),
    Unplayable,
}

impl Cell {
    pub fn parse(input: &str) -> IResult<&str, Cell> {
        alt((
            value(Cell::Playable, tag("_")),
            map(Side::parse, Cell::Played),
            value(Cell::Unplayable, tag(".")),
        ))(input)
    }
}
