use super::{direction::Direction, line::{line, Line}};
use crate::core::Position;

#[inline(always)]
pub fn lines(position: &Position, direction: &Direction, size: u8, win_length: u8) -> Vec<Line> {
    let mut lines = Vec::new();
    let delta = direction.delta();
    let delta_row = delta.0 as i16;
    let delta_column = delta.1 as i16;
    let mut row = position.0 as i16;
    let mut column = position.1 as i16;
    let _size = size as i16;
    for _ in 0..win_length {
        if let Some(line) = line(&Position(row as u8, column as u8), direction, win_length, size) {
            lines.push(line);
        }
        row -= delta_row;
        column -= delta_column;
        if row < 0 || row >= _size || column < 0 || column >= _size { break; }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lines_should_list_all_horizontal_lines_given_corner() -> Result<()> {
        let lines = lines(&Position(2, 2), &Direction::Horizontal,  3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(2, 1), Position(2, 0)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_vertical_lines_given_corner() -> Result<()> {
        let lines = lines(&Position(2, 2), &Direction::Vertical,  3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(1, 2), Position(0, 2)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_diagonal_lines_given_corner() -> Result<()> {
        let lines = lines(&Position(2, 2), &Direction::Diagonal,  3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(1, 1), Position(0, 0)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_horizontal_lines_given_edge() -> Result<()> {
        let lines = lines(&Position(2, 1), &Direction::Horizontal, 3, 2);
        let expected = vec![
            vec![Position(2, 1), Position(2, 0)],
            vec![Position(2, 2), Position(2, 1)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_vertical_lines_given_edge() -> Result<()> {
        let lines = lines(&Position(1, 0), &Direction::Vertical, 3, 2);
        let expected = vec![
            vec![Position(1, 0), Position(0, 0)],
            vec![Position(2, 0), Position(1, 0)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_no_diagonal_lines_given_edge() -> Result<()> {
        let lines = lines(&Position(1, 0), &Direction::Diagonal, 3, 3);
        let expected: Vec<Line> = vec![];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_no_anti_lines_given_edge() -> Result<()> {
        let lines = lines(&Position(1, 0), &Direction::AntiDiagonal, 3, 3);
        let expected: Vec<Line> = vec![];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_diagonal_lines_given_center() -> Result<()> {
        let lines = lines(&Position(1, 1), &Direction::Diagonal, 3, 2);
        let expected = vec![
            vec![Position(1, 1), Position(0, 0)],
            vec![Position(2, 2), Position(1, 1)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_anti_lines_given_center() -> Result<()> {
        let lines = lines(&Position(1, 1), &Direction::AntiDiagonal, 3, 2);
        let expected = vec![
            vec![Position(1, 1), Position(0, 2)],
            vec![Position(2, 0), Position(1, 1)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }

    #[test]
    fn test_lines_should_list_all_horizontal_lines_given_anti_corner() -> Result<()> {
        let lines = lines(&Position(2, 0), &Direction::Horizontal, 3, 3);
        let expected = vec![
            vec![Position(2, 2), Position(2, 1), Position(2, 0)],
        ];
        assert_eq!(lines, expected);
        Ok(())
    }
}
