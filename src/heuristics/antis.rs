use crate::core::Position;

use super::line::Line;

pub fn antis(size: u8) -> Vec<Line> {
    let usize = size as usize;
    let mut lines = Vec::with_capacity(usize);
    for d in 0..size {
        let mut line = Vec::new();
        for i in 0..=d {
            line.push(Position(i, d - i));
        }
        lines.push(line);
    }

    for d in 1..size {
        let mut line = Vec::new();
        for i in 0..(size - d) {
            line.push(Position(d + i, size - 1 - i));
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
    fn test_antis_should_return_all_antis() {
        let actual = antis(3);
        let expected = vec![
            vec![Position(0, 0)],
            vec![Position(0, 1), Position(1, 0)],
            vec![Position(0, 2), Position(1, 1), Position(2, 0)],
            vec![Position(1, 2), Position(2, 1)],
            vec![Position(2, 2)],
        ];
        assert_eq!(actual, expected);
    }
}
