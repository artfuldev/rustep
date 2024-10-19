use rustc_hash::FxHashSet;

use crate::core::{Game, Position};

use super::looker::Looker;

#[derive(Clone)]
pub struct Nearby(pub u8);
impl Nearby {
    #[inline(always)]
    fn nearby(&self, position: &Position, size: u8) -> Vec<Position> {
        let mut nearby = Vec::with_capacity((self.0 as usize * 2 + 1).pow(2));
        for i_offset in -(self.0 as isize)..=self.0 as isize {
            for j_offset in -(self.0 as isize)..=self.0 as isize {
                if let (Some(new_row), Some(new_col)) = (
                    position.0.checked_add(i_offset as u8),
                    position.1.checked_add(j_offset as u8),
                ) {
                    if new_row < size && new_col < size {
                        nearby.push(Position(new_row, new_col));
                    }
                }
            }
        }
        nearby
    }

    #[inline(always)]
    fn near_played(
        &self,
        played: &Vec<Position>,
        playable: &FxHashSet<Position>,
        size: u8,
    ) -> Vec<Position> {
        let mut seen: FxHashSet<Position> = FxHashSet::default();
        let capacity = played.len() * (2 * self.0 as usize + 1).pow(2);
        let mut moves: Vec<Position> = Vec::with_capacity(capacity);
        for position in played.iter() {
            for neighbor in self.nearby(position, size) {
                if !playable.contains(&neighbor) || !seen.insert(neighbor.clone()) {
                    continue;
                }
                moves.push(neighbor.clone());
            }
        }
        moves
    }
}

impl Looker for Nearby {
    fn moves(&mut self, game: &Game) -> Vec<Position> {
        let mid = game.size / 2;
        let center = Position(mid, mid);
        let mut moves = if game.playable.contains(&center) {
            vec![center]
        } else {
            vec![]
        };
        moves.append(&mut self.near_played(&game.moves, &game.playable, game.size));
        moves
    }
}
