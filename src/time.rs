use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::duration::Duration;

#[derive(Clone, Debug)]
pub enum Time {
    PerMove(Duration),
    Remaining(Duration),
}

impl Time {
    fn parse_remaining(input: &str) -> IResult<&str, Time> {
        let (remaining, duration) =
            preceded(tuple((tag("time-remaining"), multispace1)), Duration::parse)(input)?;
        Ok((remaining, Time::Remaining(duration)))
    }

    fn parse_per_move(input: &str) -> IResult<&str, Time> {
        let (remaining, duration) =
            preceded(tuple((tag("time"), multispace1)), Duration::parse)(input)?;
        Ok((remaining, Time::PerMove(duration)))
    }

    pub fn parse(input: &str) -> IResult<&str, Time> {
        alt((Time::parse_remaining, Time::parse_per_move))(input)
    }
}
