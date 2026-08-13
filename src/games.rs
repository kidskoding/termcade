use color_eyre::eyre::{Result, bail};
use ratatui::prelude::Color;
use ratatui::{Terminal, backend::Backend};

use crate::game::{Cel, Game, Row};

// Cabinet art is an 8x7 playfield, two columns per cell. A T piece falls into a
// three-wide notch, locks, completes the bottom row, and the row flashes and clears.
const T: Color = Color::Magenta;
const J: Color = Color::Blue;
const I: Color = Color::Cyan;
const S: Color = Color::Green;
const GAP: Color = Color::Reset;

/// Empty row.
const E: Row = &[("                ", GAP)];

/// The falling T: nub row, then the three-wide body row.
const NUB: Row = &[("        ", GAP), ("██", T), ("      ", GAP)];
const BODY: Row = &[("      ", GAP), ("██████", T), ("    ", GAP)];

/// The settled stack, top row to bottom row.
const S4: Row = &[("██", J), ("              ", GAP)];
const S5: Row = &[("████", J), ("          ", GAP), ("██", S)];
const S6: Row = &[("██", J), ("████", I), ("      ", GAP), ("████", S)];

/// Stack rows with the T overlaid as it passes through them.
const S4_NUB: Row = &[("██", J), ("      ", GAP), ("██", T), ("      ", GAP)];
const S4_BODY: Row = &[("██", J), ("    ", GAP), ("██████", T), ("    ", GAP)];
const S5_NUB: Row = &[
    ("████", J),
    ("    ", GAP),
    ("██", T),
    ("    ", GAP),
    ("██", S),
];
const S5_BODY: Row = &[
    ("████", J),
    ("  ", GAP),
    ("██████", T),
    ("  ", GAP),
    ("██", S),
];
const S6_BODY: Row = &[("██", J), ("████", I), ("██████", T), ("████", S)];

const FLASH: Row = &[("████████████████", Color::White)];

const TETRIS_ART: &[Cel] = &[
    &[NUB, BODY, E, E, S4, S5, S6],
    &[E, NUB, BODY, E, S4, S5, S6],
    &[E, E, NUB, BODY, S4, S5, S6],
    &[E, E, E, NUB, S4_BODY, S5, S6],
    &[E, E, E, E, S4_NUB, S5_BODY, S6],
    &[E, E, E, E, S4, S5_NUB, S6_BODY],
    &[E, E, E, E, S4, S5_NUB, FLASH],
    &[E, E, E, E, E, S4, S5_NUB],
];

pub const GAMES: &[Game] = &[Game {
    name: "Tetris",
    blurb: "stack falling bricks into lines",
    id: "tetris",
    hint: "controls still under construction",
    art: TETRIS_ART,
}];

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
