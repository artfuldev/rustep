use std::fmt::Display;

use num::BigUint;

fn column(mut index: u16) -> String {
    let mut result = String::new();
    loop {
        let remainder = index % 26;
        result.insert(0, (97 + remainder as u8) as char);
        index = index / 26;
        if index == 0 {
            break;
        }
        index -= 1;
    }
    result
}

pub struct Position(u16, u16);

impl Position {
    pub fn new(mov: BigUint, size: u16) -> Position {
        let index = mov.trailing_zeros().unwrap_or(0) as u16;
        Position(index / size, index % size)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", column(self.1), self.0 + 1)
    }
}
