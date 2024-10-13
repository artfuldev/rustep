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
                    return x_win_score + 1;
                }
                Termination::Won(false) => {
                    return o_win_score - 1;
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
    let mut x_win_length: u32 = 0;
    let mut o_win_length: u32 = 0;
    let mut x_win_count: i64 = 0;
    let mut o_win_count: i64 = 0;
    for win in wins(game.board.size.into(), game.win_length.into()) {
        let x_winnable = win.clone() & (game.board.played_x.clone() | game.board.playable.clone());
        if x_winnable == win {
            let win_length = (win.clone() & (game.board.played_x.clone())).count_ones() as u32;
            if x_win_length == win_length {
                x_win_count += 1;
            }
            if win_length > x_win_length {
                x_win_count = 1;
                x_win_length = win_length;
            }
        }
        let o_winnable = win.clone() & (game.board.played_o.clone() | game.board.playable.clone());
        if o_winnable == win {
            let win_length = (win.clone() & (game.board.played_o.clone())).count_ones() as u32;
            if o_win_length == win_length {
                o_win_count += 1;
            }
            if win_length > o_win_length {
                o_win_count = 1;
                o_win_length = win_length;
            }
        }
    }
    if (game.clone().win_length - (o_win_length as u8)) == 1 && !game.x_to_play {
        return o_win_score + 1;
    }
    if (game.clone().win_length - (x_win_length as u8)) == 1 && game.x_to_play {
        return x_win_score - 1;
    }
    score += 2i64.pow(x_win_length) * x_win_count;
    score -= 2i64.pow(o_win_length) * o_win_count;
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
            i64::MAX - i64::from(won.board.size).pow(2) + 1
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
    fn test_c3_is_better_than_c2() -> Result<()> {
        let (_, c2) = Game::parse("3_/2_x/3_ o")?;
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        assert!(heuristic(c3) > heuristic(c2));
        Ok(())
    }

    #[test]
    fn test_b2_is_better_than_c3() -> Result<()> {
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        let (_, b2) = Game::parse("3_/_x_/3_ o")?;
        assert!(heuristic(b2) > heuristic(c3));
        Ok(())
    }

    #[test]
    fn test_a1_is_equal_to_c3() -> Result<()> {
        let (_, a1) = Game::parse("x2_/3_/3_ o")?;
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        assert_eq!(heuristic(a1), heuristic(c3));
        Ok(())
    }

    #[test]
    fn test_b3_is_better_than_a2() -> Result<()> {
        let (_, a2) = Game::parse("xox/xo_/3_ o")?;
        let (_, b3) = Game::parse("xo_/xo_/_x_ o")?;
        assert!(heuristic(b3) > heuristic(a2));
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
        let (_, win_in_next_move) = Game::parse("2_x/_2o/_2x o")?;
        assert!(heuristic(win_in_next_move.clone()) < 0);
        Ok(())
    }

    #[test]
    fn test_game_close_to_win_by_x_should_be_winning() -> Result<()> {
        let (_, win_in_next_move) = Game::parse("oxo/_x_/x_o x")?;
        assert!(heuristic(win_in_next_move.clone()) > 0);
        Ok(())
    }
}
