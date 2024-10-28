use crate::core::Position;

use super::direction::Direction;

pub type Line = Vec<Position>;

pub fn line(start: &Position, direction: &Direction, win_length: u8, size: u8) -> Option<Line> {
    let mut line = Vec::new();
    let mut row = start.0 as i16;
    let mut column = start.1 as i16;
    let delta = direction.delta();
    let delta_row = delta.0 as i16;
    let delta_column = delta.1 as i16;
    let size = size as i16;

    for _ in 0..win_length {
        if row >= 0 && row < size && column >= 0 && column < size {
            line.push(Position(row as u8, column as u8));
        } else {
            return None;
        }
        row += delta_row;
        column += delta_column;
    }
    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_line_generates_horizontal_line() {
        let expected = Some(vec![Position(2, 2), Position(2, 1), Position(2, 0)]);
        let actual = line(&Position(2, 2), &Direction::Horizontal, 3, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_horizontal_line_of_win_length() {
        let expected = Some(vec![Position(2, 2), Position(2, 1)]);
        let actual = line(&Position(2, 2), &Direction::Horizontal, 2, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_no_horizontal_line_when_not_possible() {
        let expected = None;
        let actual = line(&Position(2, 2), &Direction::Horizontal, 4, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_vertical_line() {
        let expected = Some(vec![Position(2, 2), Position(1, 2), Position(0, 2)]);
        let actual = line(&Position(2, 2), &Direction::Vertical, 3, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_vertical_line_of_win_length() {
        let expected = Some(vec![Position(2, 2), Position(1, 2)]);
        let actual = line(&Position(2, 2), &Direction::Vertical, 2, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_no_vertical_line_when_not_possible() {
        let expected = None;
        let actual = line(&Position(2, 2), &Direction::Vertical, 4, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_diagonal_line() {
        let expected = Some(vec![Position(2, 2), Position(1, 1), Position(0, 0)]);
        let actual = line(&Position(2, 2), &Direction::Diagonal, 3, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_diagonal_line_of_win_length() {
        let expected = Some(vec![Position(2, 2), Position(1, 1)]);
        let actual = line(&Position(2, 2), &Direction::Diagonal, 2, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_no_diagonal_line_when_not_possible() {
        let expected = None;
        let actual = line(&Position(2, 2), &Direction::Diagonal, 4, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_anti_line() {
        let expected = Some(vec![Position(2, 0), Position(1, 1), Position(0, 2)]);
        let actual = line(&Position(2, 0), &Direction::AntiDiagonal, 3, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_anti_line_of_win_length() {
        let expected = Some(vec![Position(2, 0), Position(1, 1)]);
        let actual = line(&Position(2, 0), &Direction::AntiDiagonal, 2, 3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_line_generates_no_anti_line_when_not_possible() {
        let expected = None;
        let actual = line(&Position(2, 2), &Direction::AntiDiagonal, 3, 3);
        assert_eq!(actual, expected);
    }
}
