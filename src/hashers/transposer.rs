use nohash_hasher::IntSet;

use crate::core::{zobrist, Game, Position, Zobrist};

use super::Hasher;

fn transpose_once<R: Fn(u8, u8, u8) -> u8, C: Fn(u8, u8, u8) -> u8>(
    game: &Game,
    zobrist: &Zobrist,
    row: R,
    column: C,
) -> u64 {
    let size = game.size;
    let mut transposed = game.hash;
    for i in 0..size {
        for j in 0..size {
            let position = Position(i, j);
            let transposition = Position(row(i, j, size), column(i, j, size));
            match (game.get(&position), game.get(&transposition)) {
                (Some(c), Some(t)) => {
                    transposed ^= zobrist.mov(&(position.clone(), c.clone()))
                        ^ zobrist.mov(&(position, t.clone()));
                    transposed ^=
                        zobrist.mov(&(transposition.clone(), t)) ^ zobrist.mov(&(transposition, c));
                }
                _ => {}
            }
        }
    }
    transposed
}

pub struct Transposer;

impl Transposer {
    pub fn transpose(&self, game: &Game, zobrist: &Zobrist) -> Vec<u64> {
        let mut seen: IntSet<u64> = IntSet::default();
        let mut transpositions = vec![game.hash];
        let rotated_90 = transpose_once(game, zobrist, |_, j, _| j, |i, _, n| n - 1 - i);
        if seen.insert(rotated_90) {
            transpositions.push(rotated_90);
        }
        let rotated_180 = transpose_once(game, zobrist, |i, _, n| n - 1 - i, |_, j, n| n - 1 - j);
        if seen.insert(rotated_180) {
            transpositions.push(rotated_180);
        }
        let rotated_270 = transpose_once(game, zobrist, |_, j, n| n - 1 - j, |i, _, _| i);
        if seen.insert(rotated_270) {
            transpositions.push(rotated_270);
        }
        let reflected_horizontal = transpose_once(game, zobrist, |i, _, n| n - 1 - i, |_, j, _| j);
        if seen.insert(reflected_horizontal) {
            transpositions.push(reflected_horizontal);
        }
        let reflected_vertical = transpose_once(game, zobrist, |i, _, _| i, |_, j, n| n - 1 - j);
        if seen.insert(reflected_vertical) {
            transpositions.push(reflected_vertical);
        }
        let reflected_diagonal = transpose_once(game, zobrist, |_, j, _| j, |i, _, _| i);
        if seen.insert(reflected_diagonal) {
            transpositions.push(reflected_diagonal);
        }
        let reflected_anti =
            transpose_once(game, zobrist, |_, j, n| n - 1 - j, |i, _, n| n - 1 - i);
        if seen.insert(reflected_anti) {
            transpositions.push(reflected_anti);
        }
        transpositions
    }
}

impl Hasher for Transposer {
    fn hashes(&mut self, game: &crate::core::Game) -> Vec<u64> {
        let zobrist = zobrist(game.size);
        self.transpose(game, &zobrist)
    }
}
