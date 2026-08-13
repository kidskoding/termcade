use color_eyre::eyre::{Result, bail};
use ratatui::{Terminal, backend::Backend};

use crate::animations;
use crate::game::Game;

pub const GAMES: &[Game] = &[
    Game {
        name: "Tetris",
        blurb: "stack falling bricks into lines",
        id: "tetris",
        hint: "← → move · ↑ rotate · space drop",
        playable: true,
        tiles: false,
        art: animations::tetris::ART,
    },
    Game {
        name: "Snake",
        blurb: "eat, grow, don't bite yourself",
        id: "snake",
        hint: "← ↑ → ↓ steer",
        playable: false,
        tiles: false,
        art: animations::snake::ART,
    },
    Game {
        name: "Flappy",
        blurb: "one button between you and the pipes",
        id: "flappy",
        hint: "space to flap",
        playable: false,
        tiles: false,
        art: animations::flappy::ART,
    },
    Game {
        name: "Pong",
        blurb: "two paddles, one ball, no mercy",
        id: "pong",
        hint: "w/s · ↑/↓ · two players",
        playable: false,
        tiles: false,
        art: animations::pong::ART,
    },
    Game {
        name: "2048",
        blurb: "merge fast, the tiles keep coming",
        id: "2048",
        hint: "← ↑ → ↓ slide",
        playable: false,
        tiles: true,
        art: animations::merge::ART,
    },
    Game {
        name: "Breakout",
        blurb: "angle the bounce, clear the wall",
        id: "breakout",
        hint: "← → paddle",
        playable: false,
        tiles: false,
        art: animations::breakout::ART,
    },
    Game {
        name: "Galaga",
        blurb: "they break formation and dive",
        id: "galaga",
        hint: "← → move · space fire",
        playable: false,
        tiles: false,
        art: animations::galaga::ART,
    },
    Game {
        name: "Pac-Man",
        blurb: "clear the maze, outrun the ghosts",
        id: "pacman",
        hint: "← ↑ → ↓ steer",
        playable: false,
        tiles: false,
        art: animations::pacman::ART,
    },
    Game {
        name: "Invaders",
        blurb: "shoot the grid before it lands",
        id: "invaders",
        hint: "← → move · space fire",
        playable: false,
        tiles: false,
        art: animations::invaders::ART,
    },
];

pub fn launch<B>(game: &Game, terminal: &mut Terminal<B>) -> Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    match game.id {
        "tetris" => tetris::run(terminal),
        other => bail!("unknown game: {other}"),
    }
}
