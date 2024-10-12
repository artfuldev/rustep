use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[derive(Clone)]
pub enum Cell {
    Unplayable,
    Playable,
    PlayedX,
    PlayedO,
}

impl Cell {
    pub fn parse(input: &str) -> IResult<&str, Cell> {
        alt((
            value(Cell::Unplayable, tag(".")),
            value(Cell::Playable, tag("_")),
            value(Cell::PlayedX, tag("x")),
            value(Cell::PlayedO, tag("o")),
        ))(input)
    }
}
