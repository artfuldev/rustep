use super::{direction::Direction, lines::lines};
use crate::core::Position;

pub fn wins(position: &Position, size: u8, win_length: u8) -> Vec<Vec<Position>> {
    let mut wins = Vec::new();
    for direction in vec![Direction::Horizontal, Direction::Vertical, Direction::Diagonal, Direction::AntiDiagonal] {
        for line in lines(&position, &direction, size, win_length) {
            wins.push(line);
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_wins_should_list_all_lines_given_corner() -> Result<()> {
        let wins = wins(&Position(2, 2), 3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(2, 1), Position(2, 0)],
            vec![Position(2, 2), Position(1, 2), Position(0, 2)],
            vec![Position(2, 2), Position(1, 1), Position(0, 0)],
        ];
        assert_eq!(wins, expected);
        Ok(())
    }

    #[test]
    fn test_wins_should_list_all_lines_given_edge() -> Result<()> {
        let wins = wins(&Position(2, 1), 3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(2, 1), Position(2, 0)],
            vec![Position(2, 1), Position(1, 1), Position(0, 1)],
        ];
        assert_eq!(wins, expected);
        Ok(())
    }

    #[test]
    fn test_wins_should_list_all_lines_given_anti_corner() -> Result<()> {
        let wins = wins(&Position(2, 0), 3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(2, 1), Position(2, 0)],
            vec![Position(2, 0), Position(1, 0), Position(0, 0)],
            vec![Position(2, 0), Position(1, 1), Position(0, 2)],
        ];
        assert_eq!(wins, expected);
        Ok(())
    }
}
