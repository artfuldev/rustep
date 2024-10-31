pub mod core;
pub mod hashers;
pub mod heuristics;
pub mod lookers;
pub mod players;

use std::{
    error::Error,
    io::{self, BufRead, Write},
    process,
};

use crate::core::{zobrist, Command};
use hashers::Transposer;
use heuristics::{Assurer, Cached, Chance, Win};
use lookers::{Nearby, Shuffler};
use players::{Player, Thinker};
use rand::thread_rng;

const URL: &str = "https://github.com/artfuldev/rustep";

fn main() -> Result<(), Box<dyn Error>> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let mut player = Thinker::new(
        Box::new(Cached::new(
            Box::new(Win::new(Box::new(Assurer::new(Box::new(Chance))))),
            Box::new(Transposer),
        )),
        Box::new(Shuffler::new(Box::new(Nearby::new(2)), thread_rng())),
    );
    loop {
        let mut buffer = String::new();
        let mut stdin = io::stdin().lock();
        stdin.read_line(&mut buffer)?;
        let input = buffer.trim();
        match Command::parse(input) {
            Ok((_, command)) => match command {
                Command::Handshake(version) => {
                    let _ = zobrist(15);
                    let mut stdout = io::stdout().lock();
                    writeln!(stdout, "st3p version {} ok", version)?;
                    stdout.flush()?;
                }
                Command::Identify => {
                    let mut stdout = io::stdout().lock();
                    writeln!(stdout, "identify name {}", name)?;
                    writeln!(stdout, "identify version {}", version)?;
                    writeln!(stdout, "identify author {}", author)?;
                    writeln!(stdout, "identify url {}", URL)?;
                    writeln!(stdout, "identify ok")?;
                    stdout.flush()?;
                }
                Command::Move(mut game, time) => match player.best(&mut game, time) {
                    Ok(position) => {
                        let mut stdout = io::stdout().lock();
                        writeln!(stdout, "best {}", position)?;
                    }
                    Err(error) => {
                        let mut stderr = io::stderr().lock();
                        writeln!(stderr, "move error: {}", error)?;
                        stderr.flush()?;
                    }
                },
                Command::Quit => {
                    process::exit(0);
                }
            },
            _ => {
                let mut stderr = io::stderr().lock();
                writeln!(stderr, "ignoring unknown input: {}", input)?;
                stderr.flush()?;
            }
        }
    }
}
