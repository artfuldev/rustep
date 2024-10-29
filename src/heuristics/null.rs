use super::Heuristic;

pub struct Null;

impl Heuristic for Null {
    fn score(&mut self, _: &crate::core::Game) -> i64 {
        0
    }
}
