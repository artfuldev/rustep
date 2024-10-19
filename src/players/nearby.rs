use rustc_hash::FxHashSet;

use crate::core::{Game, Position};

use super::looker::Looker;

pub struct Nearby(pub u8);
impl Nearby {
    #[inline(always)]
    fn nearby(&self, position: &Position, size: u8) -> Vec<Position> {
        let mut nearby = Vec::with_capacity((self.0 as usize * 2 + 1).pow(2));
        for i_offset in -(self.0 as isize)..=self.0 as isize {
            for j_offset in -(self.0 as isize)..=self.0 as isize {
                let new_row = (position.0 as isize + i_offset) as isize;
                let new_col = (position.1 as isize + j_offset) as isize;
                if new_row >= 0
                    && new_row < size as isize
                    && new_col >= 0
                    && new_col < size as isize
                {
                    nearby.push(Position(new_row as u8, new_col as u8));
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_returns_center_when_empty() -> Result<()> {
        let (_, game) = Game::parse("5_/5_/5_/5_/5_ x")?;
        let mut nearby = Nearby(2u8);
        let moves = nearby.moves(&game);
        assert_eq!(moves, vec![Position(2, 2)]);
        Ok(())
    }

    #[test]
    fn test_returns_only_within_bounds() -> Result<()> {
        let (_, game) = Game::parse("5_/5_/2_x2_/5_/5_ x")?;
        let mut nearby = Nearby(1u8);
        let mut moves = nearby.moves(&game);
        moves.sort();
        let mut expected = vec![
            Position(2, 3),
            Position(2, 1),
            Position(1, 2),
            Position(3, 2),
            Position(1, 1),
            Position(1, 3),
            Position(3, 1),
            Position(3, 3),
        ];
        expected.sort();
        assert_eq!(moves, expected);
        Ok(())
    }

    #[test]
    fn test_returns_extremes() -> Result<()> {
        let (_, game) = Game::parse("5_/5_/2_x2_/5_/5_ x")?;
        let mut nearby = Nearby(2u8);
        let moves = nearby.moves(&game);
        assert!(moves.contains(&Position(0, 0)));
        Ok(())
    }
}
