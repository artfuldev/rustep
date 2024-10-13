use crate::core::Game;

use super::{assurances, terminated, wins, Termination};

pub fn heuristic(game: Game) -> i64 {
    let square = i64::from(game.board.size).pow(2);
    let moves_left = game.board.playable.count_ones() as i64;
    let moves_count = (game.board.played_x.count_ones() + game.board.played_o.count_ones()) as i64;
    let mut score: i64 = 0;
    if (moves_count + 1) / 2 >= i64::from(game.win_length) {
        if let Some(termination) = terminated(game.clone()) {
            match termination {
                Termination::Won(true) => {
                    return i64::MAX - square - moves_left;
                }
                Termination::Won(false) => {
                    return i64::MIN + square - moves_left;
                }
                Termination::Drawn => return 0,
            }
        }

        for (playable, played) in assurances(game.board.size.into(), game.win_length.into()) {
            if (game.board.playable.clone() & playable.clone()) != playable {
                continue;
            }
            if game.x_to_play {
                let x = played.clone() & game.board.played_x.clone();
                if x == played {
                    return i64::MAX - square + moves_left - 1;
                }
            }
            else {
                let o = played.clone() & game.board.played_o.clone();
                if o == played {
                    return i64::MIN + square - moves_left + 1;
                }
            }
        }
    }
    let mut score: i64 = 0;
    let square: i64 = i64::from(game.board.size).pow(2);
    for win in wins(game.board.size.into(), game.win_length.into()) {
        let x_winnable = win.clone() & (game.board.played_x.clone() | game.board.playable.clone());
        if x_winnable == win {
            score += square.pow(x_winnable.count_ones() as u32);
        }
        let o_winnable = win.clone() & (game.board.played_o.clone() | game.board.playable.clone());
        if o_winnable == win {
            score -= square.pow(x_winnable.count_ones() as u32);
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_game_won_by_x() -> Result<()> {
        let (_, won) = Game::parse("xox/oxo/oxx o")?;
        assert_eq!(heuristic(won.clone()), i64::MAX - i64::from(won.board.size).pow(2));
        Ok(())
    }

    #[test]
    fn test_game_won_earlier_is_better() -> Result<()> {
        let (_, later) = Game::parse("xox/oxo/oxx o")?;
        let (_, earlier) = Game::parse("3x/2o_/3_ o")?;
        assert!(heuristic(earlier) > heuristic(later));
        Ok(())
    }
}