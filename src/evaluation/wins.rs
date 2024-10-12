use memoize::memoize;
use num::BigUint;

type Win = BigUint;

#[memoize]
pub fn wins(size: usize, win_length: usize) -> Vec<Win> {
    let one: BigUint = BigUint::from(1u8);
    if win_length < 1 || win_length > size {
        return vec![];
    }

    let threshold = size - win_length + 1;
    let total_patterns = (size + threshold) * threshold * 2;
    let mut wins: Vec<Win> = Vec::with_capacity(total_patterns);
    let size_threshold_difference = size - threshold + 1;

    let mut horizontal = (one.clone() << win_length) - one.clone();
    let mut vertical = BigUint::ZERO;
    for _ in 0..win_length {
        vertical <<= size;
        vertical |= one.clone();
    }

    let mut shift: usize;
    for i in 0..(size * threshold) {
        wins.push(horizontal.clone());
        wins.push(vertical.clone());

        shift = if (i + 1) % threshold == 0 {
            size_threshold_difference
        } else {
            1
        };

        horizontal <<= shift;
        vertical <<= 1;
    }

    let mut diagonal = BigUint::ZERO;
    let size_plus_one = size + 1;
    for _ in 0..win_length {
        diagonal = (diagonal << size_plus_one) | one.clone();
    }

    let mut anti = BigUint::ZERO;
    let size_minus_one = size - 1;
    let win_length_minus_one = win_length - 1;
    for _ in 0..win_length {
        anti = (anti << size_minus_one) | (one.clone() << win_length_minus_one);
    }

    for i in 0..(threshold * threshold) {
        wins.push(diagonal.clone());
        wins.push(anti.clone());

        shift = if (i + 1) % threshold == 0 {
            size_threshold_difference
        } else {
            1
        };

        diagonal <<= &shift;
        anti <<= &shift;
    }

    wins
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;
    use pretty_assertions::assert_eq;

    fn from_binary_string(s: &str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 2).unwrap()
    }

    #[test]
    fn test_win_length_equals_size() {
        let expected: Vec<BigUint> = vec![
            "111000000",
            "000111000",
            "000000111",
            "100100100",
            "010010010",
            "001001001",
            "100010001",
            "001010100",
        ]
        .iter()
        .map(|&x| from_binary_string(x))
        .collect();

        let mut actual = wins(3, 3);
        actual.sort();
        let mut expected_sorted = expected;
        expected_sorted.sort();

        assert_eq!(actual, expected_sorted);
    }

    #[test]
    fn test_win_length_less_than_size() {
        let expected: Vec<BigUint> = vec![
            "000000011",
            "000001001",
            "000000110",
            "000010010",
            "000011000",
            "000100100",
            "000110000",
            "001001000",
            "011000000",
            "010010000",
            "110000000",
            "100100000",
            "000010001",
            "000001010",
            "000100010",
            "000010100",
            "010001000",
            "001010000",
            "100010000",
            "010100000",
        ]
        .iter()
        .map(|&x| from_binary_string(x))
        .collect();

        let mut actual = wins(3, 2);
        actual.sort();
        let mut expected_sorted = expected;
        expected_sorted.sort();

        assert_eq!(actual, expected_sorted);
    }

    #[test]
    fn test_larger_boards() {
        let expected: Vec<BigUint> = vec![
            "0000000000000000000001111",
            "0000000001000010000100001",
            "0000000000000000000011110",
            "0000000010000100001000010",
            "0000000000000000111100000",
            "0000000100001000010000100",
            "0000000000000001111000000",
            "0000001000010000100001000",
            "0000000000011110000000000",
            "0000010000100001000010000",
            "0000000000111100000000000",
            "0000100001000010000100000",
            "0000001111000000000000000",
            "0001000010000100001000000",
            "0000011110000000000000000",
            "0010000100001000010000000",
            "0111100000000000000000000",
            "0100001000010000100000000",
            "1111000000000000000000000",
            "1000010000100001000000000",
            "0000001000001000001000001",
            "0000000001000100010001000",
            "0000010000010000010000010",
            "0000000010001000100010000",
            "0100000100000100000100000",
            "0000100010001000100000000",
            "1000001000001000001000000",
            "0001000100010001000000000",
        ]
        .iter()
        .map(|&x| from_binary_string(x))
        .collect();

        let mut actual = wins(5, 4);
        actual.sort();
        let mut expected_sorted = expected;
        expected_sorted.sort();

        assert_eq!(actual, expected_sorted);
    }
}
