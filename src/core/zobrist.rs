use memoize::memoize;
use rand::{rngs::ThreadRng, thread_rng, RngCore};
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

use super::{Cell, Position, Side};

#[derive(Clone)]
pub struct Zobrist(FxHashMap<(Position, Cell), u64>, FxHashMap<Side, u64>);

impl Zobrist {
    pub fn mov(&self, key: &(Position, Cell)) -> u64 {
        if key.1 == Cell::Playable {
            0
        } else {
            self.0[&key]
        }
    }

    pub fn side(&self, side: &Side) -> u64 {
        self.1[&side]
    }

    pub fn new(size: u8, rng: &mut ThreadRng) -> Self {
        let mut used: FxHashSet<u64> = FxHashSet::default();
        let mut moves: FxHashMap<(Position, Cell), u64> =
            FxHashMap::with_capacity_and_hasher((size as usize).pow(2) * 3, FxBuildHasher);
        let mut sides: FxHashMap<Side, u64> = FxHashMap::with_capacity_and_hasher(2, FxBuildHasher);
        for side in [Side::X, Side::O] {
            let mut value = rng.next_u64();
            while used.contains(&value) {
                value = rng.next_u64();
            }
            sides.insert(side, value);
        }
        for row in 0..size {
            for column in 0..size {
                for cell in [
                    Cell::Played(Side::X),
                    Cell::Played(Side::O),
                    Cell::Unplayable,
                ] {
                    let key = (Position(row, column), cell.clone());
                    let mut value = rng.next_u64();
                    while used.contains(&value) {
                        value = rng.next_u64();
                    }
                    used.insert(value);
                    moves.insert(key.clone(), value);
                }
            }
        }
        Self(moves, sides)
    }
}

#[memoize]
pub fn zobrist(size: u8) -> Zobrist {
    Zobrist::new(size, &mut thread_rng())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::time::{Duration, Instant};

    #[test]
    #[ignore]
    fn test_zobrist_for_size_15_does_not_take_long() {
        let start = Instant::now();
        let _ = zobrist(15);
        assert!(start.elapsed() < Duration::from_micros(1500));
    }

    #[test]
    fn test_zobrist_for_size_15_returns_0_for_playable_cell() {
        let z = zobrist(15);
        assert_eq!(z.mov(&(Position(14, 5), Cell::Playable)), 0);
    }
}
