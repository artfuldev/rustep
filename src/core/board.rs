use nom::{
    bytes::complete::tag,
    character::complete::u8,
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use num::{bigint::Sign, BigInt, BigUint};

use super::{Cell, Move};

#[derive(Clone, Debug)]
pub struct Board {
    pub size: u8,
    pub playable: BigUint,
    pub played_x: BigUint,
    pub played_o: BigUint,
}

fn parse_count(input: &str) -> IResult<&str, u8> {
    let (remaining, count) = opt(u8)(input)?;
    match count {
        Some(count) => Ok((remaining, count)),
        None => Ok((remaining, 1)),
    }
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

impl Board {
    pub fn parse(input: &str) -> IResult<&str, Board> {
        let mut playable: BigUint = BigUint::ZERO;
        let mut played_x: BigUint = BigUint::ZERO;
        let mut played_o: BigUint = BigUint::ZERO;
        let (remaining, rows) =
            separated_list1(tag("/"), many1(tuple((parse_count, Cell::parse))))(input)?;
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
        let size: u8 = sqrt(square).try_into().unwrap();
        return Ok((
            remaining,
            Board {
                size,
                playable,
                played_x,
                played_o,
            },
        ));
    }

    pub fn make(self, mov: Move, x_to_play: bool) -> Self {
        let one = BigUint::from(1u8);
        let mask = (one.clone() << (self.size * self.size)) - one.clone();
        let playable = (self.playable.clone() & mask.clone())
            & ((!(BigInt::from_biguint(Sign::NoSign, mov.clone() & mask.clone())))
                .to_biguint()
                .unwrap_or(BigUint::ZERO)
                & mask);
        match x_to_play {
            true => Self {
                playable,
                played_x: &self.played_x | mov,
                ..self.clone()
            },
            false => Self {
                playable,
                played_o: &self.played_o | mov,
                ..self.clone()
            },
        }
    }
}
