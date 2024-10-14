use crate::core::Game;

use super::{terminated, wins, Heuristic, Termination};

fn score(game: &mut Game) -> i64 {
    let mut score: i64 = 0;
    let x_win_score = i64::MAX - 1000;
    let o_win_score = i64::MIN + 1000;

    if let Some(termination) = terminated(game) {
        match termination {
            Termination::Won(true) => return x_win_score,
            Termination::Won(false) => return o_win_score,
            Termination::Drawn => return 0,
        }
    }

    for win in wins(game.board.size.into(), game.win_length.into()) {
        let x_potential = win.clone() & (game.board.played_x.clone() | game.board.playable.clone());
        let o_potential = win.clone() & (game.board.played_o.clone() | game.board.playable.clone());

        let x_in_line = (win.clone() & game.board.played_x.clone()).count_ones() as u32;
        let o_in_line = (win.clone() & game.board.played_o.clone()).count_ones() as u32;

        if x_potential == win {
            score += evaluate_line(x_in_line, game.win_length, game.board.size);
        }
        if o_potential == win {
            score -= evaluate_line(o_in_line, game.win_length, game.board.size);
        }
    }

    score
}

fn evaluate_line(in_line: u32, win_length: u8, size: u8) -> i64 {
    let base_score = 2i64.pow(in_line);
    let position_score = match (win_length as u32) - in_line {
        1 => 2i64.pow((size - 1).into()),
        0 => 2i64.pow(size.into()),
        _ => 0,
    };
    base_score + position_score
}

#[derive(Clone, Copy)]
pub struct Dumb;

impl Heuristic for Dumb {
    fn score(&self, game: &mut Game) -> i64 {
        score(game)
    }
}
