use std::fmt::{Debug, Display};

type Row = u8;
type Column = u8;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position(pub Row, pub Column);

fn column(mut index: Column) -> String {
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

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", column(self.1), self.0 + 1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", column(self.1), self.0 + 1)
    }
}
