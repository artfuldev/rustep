mod assurances;
mod dumb;
mod heuristic;
mod smart;
mod termination;
mod wins;

pub use assurances::assurances;
pub use dumb::Dumb;
pub use heuristic::Heuristic;
pub use smart::Smart;
pub use termination::{terminal, terminated, Termination};
pub use wins::wins;
