use memoize::memoize;
use rustc_hash::FxHashSet;

use crate::core::{Game, Position};

use super::looker::Looker;

#[memoize]
fn nearby(position: Position, distance: u8, size: u8) -> Vec<Position> {
    let mut nearby = Vec::with_capacity((distance as usize * 2 + 1).pow(2));
    for x in -(distance as isize)..=distance as isize {
        for y in -(distance as isize)..=distance as isize {
            let i = (position.0 as isize + x) as isize;
            let j = (position.1 as isize + y) as isize;
            if i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                nearby.push(Position(i as u8, j as u8));
            }
        }
    }
    nearby
}

#[inline(always)]
fn near_played(
    played: &Vec<Position>,
    playable: &FxHashSet<Position>,
    distance: u8,
    size: u8,
) -> Vec<Position> {
    let mut seen: FxHashSet<Position> = FxHashSet::default();
    let capacity = played.len() * (2 * distance as usize + 1).pow(2);
    let mut moves: Vec<Position> = Vec::with_capacity(capacity);
    for position in played.iter() {
        for neighbor in nearby(position.clone(), distance, size) {
            if !playable.contains(&neighbor) || !seen.insert(neighbor.clone()) {
                continue;
            }
            moves.push(neighbor.clone());
        }
    }
    moves
}

pub struct Nearby;

impl Looker for Nearby {
    fn moves(&mut self, game: &Game) -> Vec<Position> {
        let mid = game.size / 2;
        let center = Position(mid, mid);
        let mut moves = if game.playable.contains(&center) {
            vec![center]
        } else {
            vec![]
        };
        moves.append(&mut near_played(
            &game.moves,
            &game.playable,
            game.win_length / 2,
            game.size,
        ));
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
        let mut nearby = Nearby;
        let moves = nearby.moves(&game);
        assert_eq!(moves, vec![Position(2, 2)]);
        Ok(())
    }

    #[test]
    fn test_returns_only_offset_of_win_length_by_2() -> Result<()> {
        let (_, mut game) = Game::parse("5_/5_/2_x2_/5_/5_ x")?;
        game.set_win_length(3);
        let mut nearby = Nearby;
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
        let mut nearby = Nearby;
        let moves = nearby.moves(&game);
        assert!(moves.contains(&Position(0, 0)));
        Ok(())
    }
}
