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

    #[test]
    fn test_win_length_equals_size() {
        let expected: Vec<BigUint> = vec![
            0b111000000u16,
            0b000111000u16,
            0b000000111u16,
            0b100100100u16,
            0b010010010u16,
            0b001001001u16,
            0b100010001u16,
            0b001010100u16,
        ]
        .iter()
        .map(|&x| BigUint::from(x))
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
            0b000000011u16,
            0b000001001u16,
            0b000000110u16,
            0b000010010u16,
            0b000011000u16,
            0b000100100u16,
            0b000110000u16,
            0b001001000u16,
            0b011000000u16,
            0b010010000u16,
            0b110000000u16,
            0b100100000u16,
            0b000010001u16,
            0b000001010u16,
            0b000100010u16,
            0b000010100u16,
            0b010001000u16,
            0b001010000u16,
            0b100010000u16,
            0b010100000u16,
        ]
        .iter()
        .map(|&x| BigUint::from(x))
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
            0b0000000000000000000001111u32,
            0b0000000001000010000100001u32,
            0b0000000000000000000011110u32,
            0b0000000010000100001000010u32,
            0b0000000000000000111100000u32,
            0b0000000100001000010000100u32,
            0b0000000000000001111000000u32,
            0b0000001000010000100001000u32,
            0b0000000000011110000000000u32,
            0b0000010000100001000010000u32,
            0b0000000000111100000000000u32,
            0b0000100001000010000100000u32,
            0b0000001111000000000000000u32,
            0b0001000010000100001000000u32,
            0b0000011110000000000000000u32,
            0b0010000100001000010000000u32,
            0b0111100000000000000000000u32,
            0b0100001000010000100000000u32,
            0b1111000000000000000000000u32,
            0b1000010000100001000000000u32,
            0b0000001000001000001000001u32,
            0b0000000001000100010001000u32,
            0b0000010000010000010000010u32,
            0b0000000010001000100010000u32,
            0b0100000100000100000100000u32,
            0b0000100010001000100000000u32,
            0b1000001000001000001000000u32,
            0b0001000100010001000000000u32,
        ]
        .iter()
        .map(|&x| BigUint::from(x))
        .collect();

        let mut actual = wins(5, 4);
        actual.sort();
        let mut expected_sorted = expected;
        expected_sorted.sort();

        assert_eq!(actual, expected_sorted);
    }
}
