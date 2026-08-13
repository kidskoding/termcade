pub mod breakout;
pub mod draw;
pub mod flappy;
pub mod galaga;
pub mod invaders;
pub mod merge;
pub mod pacman;
pub mod pong;
pub mod snake;
pub mod tetris;

pub use draw::{blocks, cel_at, cel_color};

pub(crate) const EMPTY: &str = "........";
