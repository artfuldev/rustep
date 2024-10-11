pub mod command;

use std::{
    error::Error,
    io::{self, Write},
    process::exit,
};

use command::Command;

const URL: &str = "https://github.com/artfuldev/rustep";

fn main() -> Result<(), Box<dyn Error>> {
    // Get values from Cargo.toml using environment variables set by Cargo
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
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
                Command::Quit => {
                    exit(0);
                }
            },
            _ => {}
        }
    }
}
