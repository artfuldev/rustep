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
}
