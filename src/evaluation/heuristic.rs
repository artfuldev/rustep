use crate::core::Game;

use super::{assurances, terminated, wins, Termination};

pub fn heuristic(game: Game) -> i64 {
    let moves_count = (game.board.played_x.count_ones() + game.board.played_o.count_ones()) as i64;
    let mut score: i64 = 0;
    let x_win_score = i64::MAX - moves_count;
    let o_win_score = i64::MIN + moves_count;
    if (moves_count + 1) / 2 >= i64::from(game.win_length) {
        if let Some(termination) = terminated(game.clone()) {
            match termination {
                Termination::Won(true) => {
                    return x_win_score;
                }
                Termination::Won(false) => {
                    return o_win_score;
                }
                Termination::Drawn => return 0,
            }
        }
        if game.win_length < game.board.size {
            let x_assured_win_score = x_win_score - 1;
            let o_assured_win_score = o_win_score + 1;
            for (playable, played) in assurances(game.board.size.into(), game.win_length.into()) {
                if (game.board.playable.clone() & playable.clone()) != playable {
                    continue;
                }

                let x = played.clone() & game.board.played_x.clone();
                if x == played {
                    if !game.x_to_play {
                        return x_assured_win_score;
                    } else {
                        score = score.max(score + x_assured_win_score);
                    }
                } else {
                    let o = played.clone() & game.board.played_o.clone();
                    if o == played {
                        if game.x_to_play {
                            return o_assured_win_score;
                        } else {
                            score = score.min(score - o_assured_win_score);
                        }
                    }
                }
            }
        }
    }
    let square: i64 = i64::from(game.board.size).pow(2);
    for win in wins(game.board.size.into(), game.win_length.into()) {
        let x_winnable = win.clone() & (game.board.played_x.clone() | game.board.playable.clone());
        if x_winnable == win {
            score = score.max(score + square.pow(x_winnable.count_ones() as u32));
        }
        let o_winnable = win.clone() & (game.board.played_o.clone() | game.board.playable.clone());
        if o_winnable == win {
            score = score.min(score - square.pow(o_winnable.count_ones() as u32));
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
        assert_eq!(
            heuristic(won.clone()),
            i64::MAX - i64::from(won.board.size).pow(2)
        );
        Ok(())
    }

    #[test]
    fn test_game_won_earlier_is_better() -> Result<()> {
        let (_, later) = Game::parse("xox/oxo/oxx o")?;
        let (_, earlier) = Game::parse("3x/2o_/3_ o")?;
        assert!(heuristic(earlier) > heuristic(later));
        Ok(())
    }

    #[test]
    fn test_game_won_is_better_than_assured_win() -> Result<()> {
        let (_, mut assured) = Game::parse("_x_/o2_/3_ x")?;
        assured.set_win_length(2);
        let (_, mut won) = Game::parse("_2x/o2_/3_ o")?;
        won.set_win_length(2);
        assert!(heuristic(won) > heuristic(assured));
        Ok(())
    }

    #[test]
    fn test_assured_win_only_for_played_side() -> Result<()> {
        let (_, mut assured) = Game::parse("_x_/3_/3_ o")?;
        assured.set_win_length(2);
        let (_, mut unassured) = Game::parse("_x_/_o_/3_ x")?;
        unassured.set_win_length(2);
        println!(
            "{} {}",
            -heuristic(unassured.clone()),
            heuristic(assured.clone())
        );
        assert_ne!(heuristic(unassured), heuristic(assured));
        Ok(())
    }

    #[test]
    fn test_game_close_to_win_by_o_should_be_lost() -> Result<()> {
        let (_, mut win_in_next_move) = Game::parse("4x_x5_o2xo/2x_xo2_x7_/5_x_x4_o_x/xoxoxox8_/xoxo2_o_x6_/oxox_x8_o/_o4_xo7_/7_x4_o2_/2_o4_o7_/2_x12_/_xo2_x9_/_o7_o5_/o7_o6_/_o3_2o8_/2_o6_o5_ o")?;
        win_in_next_move.set_win_length(5);
        assert!(heuristic(win_in_next_move.clone()) < 0);
        Ok(())
    }
}
