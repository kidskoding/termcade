use color_eyre::eyre::{Result, bail};
use ratatui::prelude::Color;
use ratatui::{Terminal, backend::Backend};

use crate::game::{Cel, Game, Row};

const T: Color = Color::Magenta;
const J: Color = Color::Blue;
const I: Color = Color::Cyan;
const S: Color = Color::Green;
const GAP: Color = Color::Reset;

const E: Row = &[("                ", GAP)];

const NUB: Row = &[("        ", GAP), ("██", T), ("      ", GAP)];
const BODY: Row = &[("      ", GAP), ("██████", T), ("    ", GAP)];
const NUB_R: Row = &[("        ", GAP), ("████", T), ("    ", GAP)];

const S4: Row = &[("██", J), ("              ", GAP)];
const S5: Row = &[("████", J), ("          ", GAP), ("██", S)];
const S6: Row = &[("██", J), ("████", I), ("      ", GAP), ("████", S)];

const S4_NUB: Row = &[("██", J), ("      ", GAP), ("██", T), ("      ", GAP)];
const S4_NUB_L: Row = &[("██", J), ("    ", GAP), ("████", T), ("      ", GAP)];
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
    &[E, NUB, NUB_R, NUB, S4, S5, S6],
    &[E, E, BODY, NUB, S4, S5, S6],
    &[E, E, E, NUB, S4_NUB_L, S5_NUB, S6],
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
