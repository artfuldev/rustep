use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, u8},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use super::{zobrist::zobrist, Cell, Position, Side};

#[derive(Clone, Debug)]
pub struct Game {
    pub cells: Vec<Vec<Cell>>,
    pub playable: HashSet<Position>,
    pub moves: Vec<Position>,
    pub side_to_play: Side,
    pub size: u8,
    pub win_length: u8,
    pub hash: u64,
}

fn parse_side_to_play(input: &str) -> IResult<&str, Side> {
    alt((value(Side::X, tag("x")), value(Side::O, tag("o"))))(input)
}

fn parse_count(input: &str) -> IResult<&str, u8> {
    let (remaining, count) = opt(u8)(input)?;
    match count {
        Some(count) => Ok((remaining, count)),
        None => Ok((remaining, 1)),
    }
}

pub fn parse_board(
    input: &str,
) -> IResult<&str, (Vec<Vec<Cell>>, Vec<Position>, HashSet<Position>, u8, u64)> {
    let mut moves: Vec<Position> = Vec::new();
    let mut playable: HashSet<Position> = HashSet::new();
    let mut cells: Vec<Vec<Cell>> = Vec::new();
    let mut hash: u64 = 0;
    let (remaining, groups) =
        separated_list1(tag("/"), many1(tuple((parse_count, Cell::parse))))(input)?;
    let usize = groups.len();
    let size = usize as u8;
    let zobrist = zobrist(size);
    let mut row = 0u8;
    for group in groups {
        let mut vec: Vec<Cell> = Vec::with_capacity(usize);
        let mut column = 0u8;
        for (count, cell) in group {
            for _ in 0..count {
                let position = Position(row as u8, column as u8);
                vec.push(cell.clone());
                match &cell {
                    Cell::Played(_) => {
                        hash ^= zobrist.mov(&(position.clone(), cell.clone()));
                        moves.push(position);
                    }
                    Cell::Playable => {
                        hash ^= zobrist.mov(&(position.clone(), cell.clone()));
                        playable.insert(position);
                    }
                    _ => {}
                };
                column += 1;
            }
        }
        cells.push(vec);
        row += 1;
    }
    return Ok((remaining, (cells, moves, playable, size, hash)));
}

impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        let (remaining, ((cells, played, playable, size, mut hash), side_to_play)) =
            separated_pair(parse_board, multispace1, parse_side_to_play)(input)?;
        hash ^= zobrist(size).side(&side_to_play);
        return Ok((
            remaining,
            Game {
                cells,
                size,
                moves: played,
                playable,
                side_to_play,
                win_length: size,
                hash,
            },
        ));
    }

    pub(crate) fn set_win_length(&mut self, win_length: u8) -> () {
        self.win_length = win_length;
    }

    pub fn play(&mut self, position: Position) {
        if !self.playable.contains(&position) {
            return;
        }
        let side = self.side_to_play.clone();
        let cell = Cell::Played(side.clone());
        let other = side.clone().other();
        let zobrist = zobrist(self.size);
        let Position(x, y) = position.clone();
        self.cells[x as usize][y as usize] = cell.clone();
        self.hash ^= zobrist.side(&side);
        self.hash ^= zobrist.side(&other);
        self.hash ^= zobrist.mov(&(position.clone(), Cell::Playable));
        self.hash ^= zobrist.mov(&(position.clone(), cell));
        self.playable.remove(&position);
        self.moves.push(position);
        self.side_to_play = other;
    }

    pub fn undo(&mut self) {
        match self.moves.pop() {
            None => {}
            Some(position) => {
                let side = self.side_to_play.clone();
                let other = side.clone().other();
                let cell = Cell::Played(other.clone());
                let zobrist = zobrist(self.size);
                let Position(x, y) = position.clone();
                self.cells[x as usize][y as usize] = Cell::Playable;
                self.hash ^= zobrist.side(&side);
                self.hash ^= zobrist.side(&other);
                self.hash ^= zobrist.mov(&(position.clone(), Cell::Playable));
                self.hash ^= zobrist.mov(&(position.clone(), cell));
                self.playable.insert(position);
                self.side_to_play = other;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;
    use anyhow::Result;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_parsed_game_has_right_cells() -> Result<()> {
        let (_, game) = Game::parse("3_/_x_/3_ o")?;
        assert_eq!(game.cells[1][1], Cell::Played(Side::X));
        Ok(())
    }

    #[test]
    fn test_parsed_game_has_right_size() -> Result<()> {
        let (_, game) = Game::parse("3_/_x_/3_ o")?;
        assert_eq!(game.size, 3);
        Ok(())
    }

    #[test]
    fn test_parsed_game_has_right_side_to_play() -> Result<()> {
        let (_, game) = Game::parse("3_/_x_/3_ o")?;
        assert_eq!(game.side_to_play, Side::O);
        Ok(())
    }

    #[test]
    fn test_play_removes_playable_position() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let position = Position(1, 1);
        game.play(position.clone());
        assert!(!game.playable.contains(&position));
        Ok(())
    }

    #[test]
    fn test_play_changes_side_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let clone = game.clone();
        game.play(Position(1, 1));
        assert_ne!(game.side_to_play, clone.side_to_play);
        Ok(())
    }

    #[test]
    fn test_play_adds_x_if_x_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        game.play(Position(1, 1));
        assert_eq!(game.cells[1][1], Cell::Played(Side::X));
        Ok(())
    }

    #[test]
    fn test_play_adds_o_if_o_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ o")?;
        game.play(Position(1, 1));
        assert_eq!(game.cells[1][1], Cell::Played(Side::O));
        Ok(())
    }

    #[test]
    fn test_play_adds_move() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let position = Position(1, 1);
        game.play(position.clone());
        assert!(game.moves.contains(&position));
        Ok(())
    }

    #[test]
    fn test_play_does_not_overwrite_moves() -> Result<()> {
        let (_, mut game) = Game::parse("3_/xox/3_ x")?;
        let position = Position(1, 1);
        game.play(position.clone());
        assert_eq!(game.cells[1][1], Cell::Played(Side::O));
        Ok(())
    }

    #[test]
    fn test_play_does_not_accept_unplayable_cells() -> Result<()> {
        let (_, mut game) = Game::parse("3./xox/3_ x")?;
        let position = Position(0, 1);
        game.play(position.clone());
        assert_eq!(game.cells[0][1], Cell::Unplayable);
        Ok(())
    }

    #[test]
    fn test_play_changes_hash() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let clone = game.clone();
        let position = Position(1, 1);
        game.play(position.clone());
        assert_ne!(clone.hash, game.hash);
        Ok(())
    }

    #[test]
    fn test_hash_is_equal_whether_parsed_or_played() -> Result<()> {
        let (_, parsed) = Game::parse("3_/xox/3_ o")?;
        let (_, mut played) = Game::parse("3_/3_/3_ x")?;
        played.play(Position(1, 0));
        played.play(Position(1, 1));
        played.play(Position(1, 2));
        assert_eq!(parsed.hash, played.hash);
        Ok(())
    }

    #[test]
    fn test_hash_is_equal_even_if_move_order_changes() -> Result<()> {
        let (_, parsed) = Game::parse("3_/xox/3_ o")?;
        let (_, mut played) = Game::parse("3_/3_/3_ x")?;
        played.play(Position(1, 2));
        played.play(Position(1, 1));
        played.play(Position(1, 0));
        assert_eq!(parsed.hash, played.hash);
        Ok(())
    }

    #[test]
    fn test_hash_is_unequal_if_side_to_play_changes() -> Result<()> {
        let (_, parsed) = Game::parse("3_/xox/3_ x")?;
        let (_, mut played) = Game::parse("3_/3_/3_ x")?;
        played.play(Position(1, 2));
        played.play(Position(1, 1));
        played.play(Position(1, 0));
        assert_ne!(parsed.hash, played.hash);
        Ok(())
    }

    #[test]
    fn test_undo_readds_playable() -> Result<()> {
        let (_, mut game) = Game::parse("3_/_x_/3_ o")?;
        game.undo();
        assert!(game.playable.contains(&Position(1, 1)));
        Ok(())
    }

    #[test]
    fn test_undo_changes_side_to_play() -> Result<()> {
        let (_, mut game) = Game::parse("3_/_x_/3_ o")?;
        let clone = game.clone();
        game.undo();
        assert_ne!(game.side_to_play, clone.side_to_play);
        Ok(())
    }

    #[test]
    fn test_undo_removes_last_played_move() -> Result<()> {
        let (_, mut game) = Game::parse("3_/x2_/3_ x")?;
        let position = Position(1, 1);
        game.play(position.clone());
        game.undo();
        assert!(!game.moves.contains(&position));
        Ok(())
    }

    #[test]
    fn test_undo_makes_cell_playable() -> Result<()> {
        let (_, mut game) = Game::parse("3_/xo_/3_ x")?;
        game.undo();
        assert_eq!(game.cells[1][1], Cell::Playable);
        Ok(())
    }

    #[test]
    fn test_undo_reverts_hash() -> Result<()> {
        let (_, mut game) = Game::parse("3_/3_/3_ x")?;
        let clone = game.clone();
        game.play(Position(1, 1));
        game.undo();
        assert_eq!(game.hash, clone.hash);
        Ok(())
    }

    #[test]
    fn test_parse_takes_little_time_for_15x15_board() -> Result<()> {
        let start = Instant::now();
        let _ = Game::parse("15_/15_/15_/15_/15_/15_/15_/15_/15_/15_/15_/15_/15_/15_/15_ x")?;
        assert!(start.elapsed() < Duration::from_micros(1500));
        Ok(())
    }
}
