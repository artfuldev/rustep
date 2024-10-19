use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace1, u8};
use nom::combinator::{opt, value, verify};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use super::Game;
use super::Time;

#[derive(Clone)]
pub enum Command {
    Handshake(u8),
    Move(Game, Option<Time>),
    Identify,
    Quit,
}

fn parse_win_length(input: &str) -> IResult<&str, u8> {
    preceded(tuple((tag("win-length"), multispace1)), u8)(input)
}

fn parse_move(input: &str) -> IResult<&str, Command> {
    let (remaining, (mut game, time, win_length)) = preceded(
        tuple((tag("move"), multispace1)),
        tuple((
            Game::parse,
            opt(preceded(multispace1, Time::parse)),
            opt(preceded(multispace1, parse_win_length)),
        )),
    )(input)?;
    match win_length {
        Some(win_length) => {
            game.set_win_length(win_length);
        }
        _ => {}
    }
    Ok((remaining, Command::Move(game, time)))
}

fn parse_handshake(input: &str) -> IResult<&str, Command> {
    let (remaining, version) = preceded(
        tuple((tag("st3p"), multispace1, tag("version"), multispace1)),
        verify(u8, |version: &u8| *version >= 1 && *version <= 2),
    )(input)?;
    Ok((remaining, Command::Handshake(version)))
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Command> {
        alt((
            parse_handshake,
            value(Command::Identify, tag("identify")),
            parse_move,
            value(Command::Quit, tag("quit")),
        ))(input)
    }
}
