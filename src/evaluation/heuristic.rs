use num::BigUint;

use crate::core::Game;

use super::{assurances, terminated, wins, Termination};

pub fn heuristic(game: Game) -> i64 {
    if let Some(termination) = terminated(game.clone()) {
        match termination {
            Termination::Won(true) => {
                return i64::MAX - (i64::from(game.board.size).pow(2))
                    + (game.board.playable.count_ones() as i64);
            }
            Termination::Won(false) => {
                return i64::MIN + (i64::from(game.board.size).pow(2))
                    - (game.board.playable.count_ones() as i64);
            }
            Termination::Drawn => return 0,
        }
    }

    for (playable, played) in assurances(game.board.size.into(), game.win_length.into()) {
        if (game.board.playable.clone() & playable.clone()) != playable {
            continue;
        }
        let x = played.clone() & game.board.played_x.clone();
        if x == played {
            return (i64::MAX / 2) - i64::from(game.board.size).pow(2)
                + (game.board.playable.clone().count_ones() as i64);
        }
        let o = played.clone() & game.board.played_o.clone();
        if o == played {
            return (i64::MIN / 2) + (i64::from(game.board.size).pow(2))
                - (game.board.playable.count_ones() as i64);
        }
    }

    let mut score: i64 = 0;
    for win in wins(game.board.size.into(), game.win_length.into()) {
        let x_winnable = win.clone() & game.board.played_x.clone();
        if (x_winnable.clone() & game.board.played_o.clone()) == BigUint::ZERO {
            score += x_winnable.count_ones() as i64;
        }
        let o_winnable = win.clone() & game.board.played_o.clone();
        if (o_winnable.clone() & game.board.played_x.clone()) == BigUint::ZERO {
            score -= o_winnable.count_ones() as i64;
        }
    }
    score
}
