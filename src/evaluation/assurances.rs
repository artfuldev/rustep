use memoize::memoize;
use num::BigUint;

type Playable = BigUint;
type Played = BigUint;
type Assurance = (Playable, Played);

#[memoize]
pub fn assurances(size: usize, assurance: usize) -> Vec<Assurance> {
    let one: BigUint = BigUint::from(1u8);
    if assurance < 1 || assurance >= size {
        return vec![];
    }
    let win_length = assurance + 1;
    let threshold = size - win_length + 1;
    let mut assurances: Vec<Assurance> = Vec::with_capacity((size + threshold) * threshold * 2);

    let size_threshold_difference = size - threshold + 1;

    let mut horizontal_playable = (one.clone() << assurance) + one.clone();
    let mut horizontal_played = (one.clone() << assurance) - 2u8;
    let mut vertical_playable = one.clone();
    let mut vertical_played = BigUint::ZERO;
    for _ in 1..assurance {
        vertical_playable = vertical_playable.clone() << size;
        vertical_played = (vertical_played.clone() << size) | one.clone();
    }
    vertical_playable = (vertical_playable.clone() << size) | one.clone();
    vertical_played = vertical_played.clone() << size;
    let mut shift: usize;
    for i in 0..(size * threshold) {
        assurances.push((horizontal_playable.clone(), horizontal_played.clone()));
        assurances.push((vertical_playable.clone(), vertical_played.clone()));
        shift = if (i + 1) % threshold == 0 {
            size_threshold_difference
        } else {
            1
        };
        horizontal_playable <<= shift;
        horizontal_played <<= shift;
        vertical_playable <<= 1;
        vertical_played <<= 1;
    }
    let mut diagonal_playable = one.clone();
    let mut diagonal_played = BigUint::ZERO;
    let size_plus_one = size + 1;
    for _ in 1..assurance {
        diagonal_playable = diagonal_playable.clone() << size_plus_one;
        diagonal_played = (diagonal_played.clone() << size_plus_one) | one.clone();
    }
    diagonal_playable = (diagonal_playable.clone() << size_plus_one) | one.clone();
    diagonal_played = diagonal_played.clone() << size_plus_one;
    let size_minus_one = size - 1;
    let win_length_minus_one = win_length - 1;
    let mut anti_playable = one.clone() << win_length_minus_one;
    let mut anti_played = BigUint::ZERO;
    for _ in 1..assurance {
        anti_playable = anti_playable.clone() << size_minus_one;
        anti_played =
            (anti_played.clone() << size_minus_one) | (one.clone() << win_length_minus_one);
    }
    anti_playable =
        (anti_playable.clone() << size_minus_one) | (one.clone() << win_length_minus_one);
    anti_played = anti_played.clone() << size_minus_one;
    for i in 0..(threshold * threshold) {
        assurances.push((diagonal_playable.clone(), diagonal_played.clone()));
        assurances.push((anti_playable.clone(), anti_played.clone()));
        shift = if (i + 1) % threshold == 0 {
            size_threshold_difference
        } else {
            1
        };
        diagonal_playable <<= shift;
        diagonal_played <<= shift;
        anti_playable <<= shift;
        anti_played <<= shift;
    }
    assurances
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn from_binary_string(s: &str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 2).unwrap()
    }

    #[test]
    fn test_assurances_size_minus_one() {
        let expected: Vec<Assurance> = vec![
            ("101000000", "010000000"),
            ("000101000", "000010000"),
            ("000000101", "000000010"),
            ("100000100", "000100000"),
            ("010000010", "000010000"),
            ("001000001", "000001000"),
            ("100000001", "000010000"),
            ("001000100", "000010000"),
        ]
        .iter()
        .map(|&(x, y)| (from_binary_string(x), from_binary_string(y)))
        .collect();

        let mut actual = assurances(3, 2);
        actual.sort_by(|a, b| a.0.cmp(&b.0)); // You may sort by different elements
        let mut expected_sorted = expected;
        expected_sorted.sort_by(|a, b| a.0.cmp(&b.0)); // Sorting by first tuple element

        assert_eq!(actual, expected_sorted);
    }

    #[test]
    fn test_assurances_larger_boards() {
        let expected: Vec<Assurance> = vec![
            ("0000000000000000000001001", "0000000000000000000000110"),
            ("0000000001000000000000001", "0000000000000010000100000"),
            ("0000000000000000000010010", "0000000000000000000001100"),
            ("0000000010000000000000010", "0000000000000100001000000"),
            ("0000000000000000100100000", "0000000000000000011000000"),
            ("0000000100000000000000100", "0000000000001000010000000"),
            ("0000000000000001001000000", "0000000000000000110000000"),
            ("0000001000000000000001000", "0000000000010000100000000"),
            ("0000000000010010000000000", "0000000000001100000000000"),
            ("0000010000000000000010000", "0000000000100001000000000"),
            ("0000000000100100000000000", "0000000000011000000000000"),
            ("0000100000000000000100000", "0000000001000010000000000"),
            ("0000001001000000000000000", "0000000110000000000000000"),
            ("0001000000000000001000000", "0000000010000100000000000"),
            ("0000010010000000000000000", "0000001100000000000000000"),
            ("0010000000000000010000000", "0000000100001000000000000"),
            ("0100100000000000000000000", "0011000000000000000000000"),
            ("0100000000000000100000000", "0000001000010000000000000"),
            ("1001000000000000000000000", "0110000000000000000000000"),
            ("1000000000000001000000000", "0000010000100000000000000"),
            ("0000001000000000000000001", "0000000000001000001000000"),
            ("0000000001000000000001000", "0000000000000100010000000"),
            ("0000010000000000000000010", "0000000000010000010000000"),
            ("0000000010000000000010000", "0000000000001000100000000"),
            ("0100000000000000000100000", "0000000100000100000000000"),
            ("0000100000000000100000000", "0000000010001000000000000"),
            ("1000000000000000001000000", "0000001000001000000000000"),
            ("0001000000000001000000000", "0000000100010000000000000"),
        ]
        .iter()
        .map(|&(x, y)| (from_binary_string(x), from_binary_string(y)))
        .collect();

        let mut actual = assurances(5, 3);
        actual.sort_by(|a, b| a.0.cmp(&b.0)); // You may sort by different elements
        let mut expected_sorted = expected;
        expected_sorted.sort_by(|a, b| a.0.cmp(&b.0)); // Sorting by first tuple element

        assert_eq!(actual, expected_sorted);
    }
}
