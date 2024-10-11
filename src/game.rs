use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, u8},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};
use num_bigint::BigUint;

#[derive(Clone, Debug)]
pub struct Game {
    pub size: u8,
    pub win_length: u8,
    pub x_to_play: bool,
    pub playable: BigUint,
    pub played_x: BigUint,
    pub played_o: BigUint,
}

#[derive(Clone)]
enum Cell {
    Unplayable,
    Playable,
    PlayedX,
    PlayedO,
}

impl Game {
    fn parse_count(input: &str) -> IResult<&str, u8> {
        let (remaining, count) = opt(u8)(input)?;
        match count {
            Some(count) => Ok((remaining, count)),
            None => Ok((remaining, 1)),
        }
    }

    fn parse_x_to_play(input: &str) -> IResult<&str, bool> {
        alt((value(true, tag("x")), value(false, tag("o"))))(input)
    }

    fn parse_cell(input: &str) -> IResult<&str, Cell> {
        alt((
            value(Cell::Unplayable, tag(".")),
            value(Cell::Playable, tag("_")),
            value(Cell::PlayedX, tag("x")),
            value(Cell::PlayedO, tag("o")),
        ))(input)
    }

    fn sqrt(n: usize) -> usize {
        let _ = n == 0 && return n;
        let mut s = (n as f64).sqrt() as usize;
        s = (s + n / s) >> 1;
        if s * s > n {
            s - 1
        } else {
            s
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Game> {
        let mut playable: BigUint = BigUint::ZERO;
        let mut played_x: BigUint = BigUint::ZERO;
        let mut played_o: BigUint = BigUint::ZERO;
        let (remaining, (rows, x_to_play)) = separated_pair(
            separated_list1(
                tag("/"),
                many1(tuple((Game::parse_count, Game::parse_cell))),
            ),
            multispace1,
            Game::parse_x_to_play,
        )(input)?;
        let mut square = 0;
        for groups in rows {
            for (count, cell) in groups {
                for _ in 1..=count {
                    square += 1;
                    playable <<= 1;
                    played_x <<= 1;
                    played_o <<= 1;
                    match cell {
                        Cell::Playable => {
                            playable += 1u8;
                        }
                        Cell::PlayedX => {
                            played_x += 1u8;
                        }
                        Cell::PlayedO => {
                            played_o += 1u8;
                        }
                        _ => {}
                    }
                }
            }
        }
        let size: u8 = Game::sqrt(square).try_into().unwrap();
        return Ok((
            remaining,
            Game {
                size,
                playable,
                played_x,
                played_o,
                x_to_play,
                win_length: size,
            },
        ));
    }

    pub(crate) fn set_win_length(&mut self, win_length: u8) -> () {
        self.win_length = win_length;
    }
}
