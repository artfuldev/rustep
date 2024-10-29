use crate::core::Position;

use super::line::Line;

pub fn rows(size: u8) -> Vec<Line> {
    let usize = size as usize;
    let mut lines = Vec::with_capacity(usize);
    for i in 0..size {
        let mut line = Vec::with_capacity(usize);
        for j in 0..size {
            line.push(Position(i, j));
        }
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rows_should_return_all_rows() {
        let actual = rows(3);
        let expected = vec![
            vec![Position(0, 0), Position(0, 1), Position(0, 2)],
            vec![Position(1, 0), Position(1, 1), Position(1, 2)],
            vec![Position(2, 0), Position(2, 1), Position(2, 2)]
        ];
        assert_eq!(actual, expected);
    }
}