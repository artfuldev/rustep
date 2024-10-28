pub enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    AntiDiagonal,
}

impl Direction {
    #[inline(always)]
    pub fn delta(&self) -> (i8, i8) {
        match &self {
            Self::Horizontal => (0, -1),
            Self::Vertical => (-1, 0),
            Self::Diagonal => (-1, -1),
            Self::AntiDiagonal => (-1, 1),
        }
    }
}
