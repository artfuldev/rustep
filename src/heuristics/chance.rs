use std::collections::{HashMap, HashSet};

use memoize::memoize;

use crate::core::{Cell, Game, Position, Side};

use super::{line::Line, wins::wins, Heuristic};

#[memoize]
fn winning_lines(size: u8, win_length: u8) -> Vec<Line> {
    let mut seen = HashSet::new();
    let mut lines = vec![];
    for i in 0..size {
        for j in 0..size {
            for line in wins(&Position(i, j), size, win_length) {
                if seen.insert(line.clone()) {
                    lines.push(line);
                }
            }
        }
    }
    lines
}

fn winnable(line: &Line, game: &Game) -> Option<(Side, u8)> {
    let mut side: Option<Side> = None;
    let mut count = 0u8;
    for position in line {
        match game.get(position) {
            None => {
                return None;
            }
            Some(Cell::Played(played)) => {
                if None == side {
                    side = Some(played.clone());
                }
                if Some(played) != side {
                    return None;
                }
                count += 1;
            }
            _ => {}
        }
    }
    if count == 0 {
        return None;
    }
    side.map(|side| (side, count))
}

pub struct Chance;

impl Heuristic for Chance {
    fn score(&mut self, game: &Game) -> i64 {
        let mut x_win_lengths = HashMap::new();
        let mut o_win_lengths = HashMap::new();
        for i in 0..game.win_length {
            x_win_lengths.insert(i, 0u8);
            o_win_lengths.insert(i, 0u8);
        }
        for line in winning_lines(game.size, game.win_length) {
            match winnable(&line, game) {
                Some((side, count)) => match side {
                    Side::X => {
                        if count == game.win_length {
                            return i64::MAX - (game.moves.len() as i64);
                        }
                        x_win_lengths
                            .insert(count, x_win_lengths.get(&count).expect("warmed up") + 1u8);
                    }
                    Side::O => {
                        if count == game.win_length {
                            return i64::MIN + (game.moves.len() as i64);
                        }
                        o_win_lengths
                            .insert(count, o_win_lengths.get(&count).expect("warmed up") + 1u8);
                    }
                },
                None => {}
            }
        }
        let imminent = game.win_length - 1u8;
        let x_imminent_win_chances = *x_win_lengths.get(&imminent).expect("warmed up");
        let o_imminent_win_chances = *o_win_lengths.get(&imminent).expect("warmed up");
        if game.side_to_play == Side::X && (x_imminent_win_chances > 0u8) {
            return i64::MAX - (game.moves.len() as i64) - 1;
        }
        if game.side_to_play == Side::O && (o_imminent_win_chances > 0u8) {
            return i64::MIN + (game.moves.len() as i64) + 1;
        }
        match (
            x_imminent_win_chances > 1u8,
            o_imminent_win_chances > 1u8,
            game.side_to_play.clone(),
        ) {
            (true, false, Side::O) => {
                return i64::MAX - (game.moves.len() as i64) - 2;
            }
            (false, true, Side::X) => {
                return i64::MIN + (game.moves.len() as i64) + 2;
            }
            _ => {}
        }
        let mut score = 0;
        for i in imminent..0 {
            let x_wins = *x_win_lengths.get(&i).expect("warmed up") as i64;
            let o_wins = *o_win_lengths.get(&i).expect("warmed up") as i64;
            let local_score = (x_wins - o_wins) * 2i64.pow(2 * (i as u32));
            score += local_score;
        }
        score
    }
}
