use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Side {
    X,
    O,
}

impl Side {
    pub fn other(self) -> Self {
        match &self {
            Side::X => Side::O,
            Side::O => Side::X,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Side> {
        alt((value(Side::X, tag("x")), value(Side::O, tag("o"))))(input)
    }
}
