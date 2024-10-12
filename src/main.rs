pub mod core;
pub mod evaluation;
pub mod players;

use std::{
    error::Error,
    io::{self, BufRead, Write},
    process,
};

use core::Command;
use players::{Player, Random};
use rand::thread_rng;

const URL: &str = "https://github.com/artfuldev/rustep";

fn main() -> Result<(), Box<dyn Error>> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let player = &Random::new(thread_rng());
    loop {
        let mut buffer = String::new();
        let mut stdin = io::stdin().lock();
        stdin.read_line(&mut buffer)?;
        let input = buffer.trim();
        match Command::parse(input) {
            Ok((_, command)) => match command {
                Command::Handshake(version) => {
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
                Command::Move(game, time) => {
                    let position = player.clone().best(game, time);
                    match position {
                        Ok(position) => {
                            let mut stdout = io::stdout().lock();
                            writeln!(stdout, "best {}", position)?;
                        }
                        Err(error) => {
                            let mut stderr = io::stderr().lock();
                            writeln!(stderr, "{}", error)?;
                            stderr.flush()?;
                        }
                    }
                }
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
