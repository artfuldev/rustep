use std::vec;

use num::BigUint;

use super::Game;

pub type Move = BigUint;

pub fn moves(game: Game) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut mov: Move = BigUint::from(1u8) << game.board.playable.trailing_zeros().unwrap_or(0);
    while mov.clone() < game.board.playable.clone() {
        moves.push(mov.clone());
        mov <<= 1;
    }
    return moves;
}

#[cfg(test)]
mod tests {
    use crate::core::Position;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_moves_count() -> Result<()> {
        let (_, game) = Game::parse("xox/oxo/3_ x")?;
        let moves = moves(game.clone());
        assert_eq!(moves.len(), 3);
        Ok(())
    }

    #[test]
    fn test_move_names() -> Result<()> {
        let (_, game) = Game::parse("xox/oxo/3_ x")?;
        let moves = moves(game.clone()).clone();
        let mut positions: Vec<String> = moves
            .iter()
            .map(|mov| Position::new(mov.clone(), game.board.size.into()))
            .map(|pos| format!("{}", pos))
            .collect();
        positions.sort();
        assert_eq!(positions.join(", "), "a3, b3, c3");
        Ok(())
    }
}
