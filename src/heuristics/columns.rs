use crate::core::Position;

use super::line::Line;

pub fn columns(size: u8) -> Vec<Line> {
    let usize = size as usize;
    let mut lines = Vec::with_capacity(usize);
    for i in 0..size {
        let mut line = Vec::with_capacity(usize);
        for j in 0..size {
            line.push(Position(j, i));
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
    fn test_columns_should_return_all_columns() {
        let actual = columns(3);
        let expected = vec![
            vec![Position(0, 0), Position(1, 0), Position(2, 0)],
            vec![Position(0, 1), Position(1, 1), Position(2, 1)],
            vec![Position(0, 2), Position(1, 2), Position(2, 2)]
        ];
        assert_eq!(actual, expected);
    }
}