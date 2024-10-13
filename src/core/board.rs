use std::fmt::{self, Display};

use nom::{
    bytes::complete::tag,
    character::complete::u8,
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use num::BigUint;

use super::{Cell, Move};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
        let full_mask = (one.clone() << (self.size * self.size)) - one.clone();
        let inverted_move = &full_mask ^ mov.clone();
        let playable = self.clone().playable & inverted_move;
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

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.size as usize;
        let total_positions = size * size;

        let playable_str = self.playable.to_str_radix(2).pad_start(total_positions, '0');
        let x_str = self.played_x.to_str_radix(2).pad_start(total_positions, '0');
        let o_str = self.played_o.to_str_radix(2).pad_start(total_positions, '0');

        let mut result = String::new();

        for i in 0..total_positions {
            let character = if x_str.chars().nth(i).unwrap() == '1' {
                'x'
            } else if o_str.chars().nth(i).unwrap() == '1' {
                'o'
            } else if playable_str.chars().nth(i).unwrap() == '1' {
                '_'
            } else {
                '.'
            };

            // Append character and determine if a newline is needed
            result.push(character);
            if (i + 1) % size == 0 {
                result.push('\n');
            } else {
                result.push(' ');
            }
        }

        write!(f, "{}", result)
    }
}

trait PadStart {
    fn pad_start(&self, size: usize, with: char) -> String;
}

impl PadStart for String {
    fn pad_start(&self, size: usize, with: char) -> String {
        format!("{:0>size$}", self, size = size).replace('0', &String::from(with))
    }
}

impl PadStart for &str {
    fn pad_start(&self, size: usize, with: char) -> String {
        self.to_string().pad_start(size, with)
    }
}

impl PadStart for BigUint {
    fn pad_start(&self, size: usize, with: char) -> String {
        self.to_str_radix(2).pad_start(size, with)
    }
}