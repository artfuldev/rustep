use super::{assurances, terminated, wins, Heuristic, Termination};

#[derive(Clone, Copy)]
pub struct Smart;

impl Heuristic for Smart {
    fn score(&self, game: crate::core::Game) -> i64 {
        let moves_count =
            (game.board.played_x.count_ones() + game.board.played_o.count_ones()) as i64;
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
        }
        let mut x_win_length: u32 = 0;
        let mut o_win_length: u32 = 0;
        let mut x_win_count: i64 = 0;
        let mut o_win_count: i64 = 0;
        for win in wins(game.board.size.into(), game.win_length.into()) {
            let x_winnable =
                win.clone() & (game.board.played_x.clone() | game.board.playable.clone());
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
            let o_winnable =
                win.clone() & (game.board.played_o.clone() | game.board.playable.clone());
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
        let x_assured_win_score = x_win_score - 1;
        let o_assured_win_score = o_win_score + 1;
        if (game.clone().win_length - (o_win_length as u8)) == 1 && !game.x_to_play {
            return o_assured_win_score;
        }
        if (game.clone().win_length - (x_win_length as u8)) == 1 && game.x_to_play {
            return x_assured_win_score;
        }
        if moves_count / 2 >= i64::from(game.win_length) && game.win_length < game.board.size {
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
                }
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
        score += 2i64.pow(x_win_length) * x_win_count;
        score -= 2i64.pow(o_win_length) * o_win_count;
        score
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Game;

    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_game_won_by_x() -> Result<()> {
        let (_, won) = Game::parse("xox/oxo/oxx o")?;
        assert_eq!(
            Smart.score(won.clone()),
            i64::MAX - i64::from(won.board.size).pow(2) + 1
        );
        Ok(())
    }

    #[test]
    fn test_game_won_earlier_is_better() -> Result<()> {
        let (_, later) = Game::parse("xox/oxo/oxx o")?;
        let (_, earlier) = Game::parse("3x/2o_/3_ o")?;
        assert!(Smart.score(earlier) > Smart.score(later));
        Ok(())
    }

    #[test]
    fn test_game_won_is_better_than_assured_win() -> Result<()> {
        let (_, mut assured) = Game::parse("_x_/o2_/3_ x")?;
        assured.set_win_length(2);
        let (_, mut won) = Game::parse("_2x/o2_/3_ o")?;
        won.set_win_length(2);
        assert!(Smart.score(won) > Smart.score(assured));
        Ok(())
    }

    #[test]
    fn test_c3_is_better_than_c2() -> Result<()> {
        let (_, c2) = Game::parse("3_/2_x/3_ o")?;
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        assert!(Smart.score(c3) > Smart.score(c2));
        Ok(())
    }

    #[test]
    fn test_b2_is_better_than_c3() -> Result<()> {
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        let (_, b2) = Game::parse("3_/_x_/3_ o")?;
        assert!(Smart.score(b2) > Smart.score(c3));
        Ok(())
    }

    #[test]
    fn test_a1_is_equal_to_c3() -> Result<()> {
        let (_, a1) = Game::parse("x2_/3_/3_ o")?;
        let (_, c3) = Game::parse("3_/3_/2_x o")?;
        assert_eq!(Smart.score(a1), Smart.score(c3));
        Ok(())
    }

    #[test]
    fn test_b3_is_better_than_a2() -> Result<()> {
        let (_, a2) = Game::parse("xox/xo_/3_ o")?;
        let (_, b3) = Game::parse("xo_/xo_/_x_ o")?;
        assert!(Smart.score(b3) > Smart.score(a2));
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
            -Smart.score(unassured.clone()),
            Smart.score(assured.clone())
        );
        assert_ne!(Smart.score(unassured), Smart.score(assured));
        Ok(())
    }

    #[test]
    fn test_game_close_to_win_by_o_should_be_lost() -> Result<()> {
        let (_, win_in_next_move) = Game::parse("2_x/_2o/_2x o")?;
        assert!(Smart.score(win_in_next_move.clone()) < 0);
        Ok(())
    }

    #[test]
    fn test_larger_game_with_win_length_close_to_win_by_o_should_be_lost() -> Result<()> {
        let (_, mut win_in_next_move) = Game::parse("2o_x_/2_x2_/2_x2_/5_/5_ o")?;
        win_in_next_move.set_win_length(3);
        assert!(Smart.score(win_in_next_move.clone()) < 0);
        Ok(())
    }

    #[test]
    fn test_game_with_win_length_close_to_win_by_o_should_not_be_lost() -> Result<()> {
        let (_, mut win_in_next_move) = Game::parse("2o2x_/5_/2_x2_/5_/5_ o")?;
        win_in_next_move.set_win_length(3);
        assert!(Smart.score(win_in_next_move.clone()) >= 0);
        Ok(())
    }

    #[test]
    fn test_game_close_to_win_by_x_should_be_winning() -> Result<()> {
        let (_, win_in_next_move) = Game::parse("oxo/_x_/x_o x")?;
        assert!(Smart.score(win_in_next_move.clone()) > 0);
        Ok(())
    }

    #[test]
    fn test_game_o_at_e4_loses() -> Result<()> {
        let (_, mut lost) = Game::parse("3o3x4o2x3o/2xoxo2xo2xoxo2x/ox2oxox_x2o_o2x/xox2o2_3ox4o/4ox_oxo_2ox_o/_o2x2o_x_xo2_x_/x_2o5_o_xo2_/3_xo2_ox6_/_x_o2_x7_x/4_x6_2x2_/2_x_x5_x4_/xo3_x4_x2_x_/3_x7_x3_/oxo6_x4_x/4_x7_x2_ o")?;
        lost.set_win_length(5);
        assert!(Smart.score(lost.clone()) < 0);
        Ok(())
    }

    #[test]
    fn test_game_x_at_e4_is_still_in_the_game() -> Result<()> {
        let (_, mut still) = Game::parse("3o3x4o2x3o/2xoxo2xo2xoxo2x/ox2oxox_x2o_o2x/xoxox2_3ox4o/4ox_oxo_2ox_o/_o2x2o_x_xo2_x_/x_2o5_o_xo2_/3_xo2_ox6_/_x_o2_x7_x/4_x6_2x2_/2_x_x5_x4_/xo3_x4_x2_x_/3_x7_x3_/oxo6_x4_x/4_x7_x2_ o")?;
        still.set_win_length(5);
        assert!(Smart.score(still.clone()) >= 0);
        Ok(())
    }
}
