use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, u8};
use nom::combinator::{value, verify};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[derive(Clone)]
pub(crate) enum Command {
    Handshake(u8),
    Identify,
    Quit,
}

impl Command {
    fn parse_handshake(input: &str) -> IResult<&str, Command> {
        let (remaining, version) = preceded(
            tuple((tag("st3p"), multispace1, tag("version"), multispace1)),
            verify(u8, |version: &u8| *version >= 1 && *version <= 2),
        )(input)?;
        Ok((remaining, Command::Handshake(version)))
    }

    pub fn parse(input: &str) -> IResult<&str, Command> {
        alt((
            Command::parse_handshake,
            value(Command::Identify, tag("identify")),
            value(Command::Quit, tag("quit")),
        ))(input)
    }
}
