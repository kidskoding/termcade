use color_eyre::eyre::{Result, bail};
use ratatui::{Terminal, backend::Backend};

use crate::game::{Cel, Game};

const EMPTY: &str = "........";

const TETRIS_ART: &[Cel] = &[
    &[
        "....m...", "...mmm..", EMPTY, EMPTY, "b.......", "bb.....g", "bcc...gg",
    ],
    &[
        EMPTY, "....m...", "....mm..", "....m...", "b.......", "bb.....g", "bcc...gg",
    ],
    &[
        EMPTY, EMPTY, "...mmm..", "....m...", "b.......", "bb.....g", "bcc...gg",
    ],
    &[
        EMPTY, EMPTY, EMPTY, "....m...", "b..mm...", "bb..m..g", "bcc...gg",
    ],
    &[
        EMPTY, EMPTY, EMPTY, EMPTY, "b...m...", "bb.mmm.g", "bcc...gg",
    ],
    &[
        EMPTY, EMPTY, EMPTY, EMPTY, "b.......", "bb..m..g", "bccmmmgg",
    ],
    &[
        EMPTY, EMPTY, EMPTY, EMPTY, "b.......", "bb..m..g", "wwwwwwww",
    ],
    &[EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, "b.......", "bb..m..g"],
];

const SNAKE_ART: &[Cel] = &[
    &[EMPTY, EMPTY, EMPTY, ".ggg..r.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "..ggg.r.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "...gggr.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "....ggg.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, "..r.....", EMPTY, "...gggg.", EMPTY, EMPTY, EMPTY],
    &[
        EMPTY, "..r.....", "......g.", "...ggg..", EMPTY, EMPTY, EMPTY,
    ],
];

const FLAPPY_ART: &[Cel] = &[
    &[
        "......g.", "......g.", "......g.", EMPTY, "..y...g.", "......g.", "......g.",
    ],
    &[
        ".....g..", ".....g..", ".....g..", "..y.....", ".....g..", ".....g..", ".....g..",
    ],
    &[
        "....g...", "....g...", "..y.g...", EMPTY, "....g...", "....g...", "....g...",
    ],
    &[
        "...g....", "...g....", "...g....", "..y.....", "...g....", "...g....", "...g....",
    ],
    &[
        "..g.....", "..g.....", "..g.....", "..y.....", "..g.....", "..g.....", "..g.....",
    ],
    &[
        ".g......", ".g......", ".g......", "..y.....", ".g......", ".g......", ".g......",
    ],
];

const PONG_ART: &[Cel] = &[
    &[
        EMPTY, EMPTY, "c......c", "c.w....c", "c......c", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "c..w...c", "c......c", "c......c", EMPTY, EMPTY,
    ],
    &[
        EMPTY, "....w...", "c......c", "c......c", "c......c", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "c....w.c", "c......c", "c......c", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "c.......", "c.....wc", "c......c", ".......c", EMPTY,
    ],
    &[
        EMPTY, EMPTY, "c.......", "c......c", "c....w.c", ".......c", EMPTY,
    ],
];

const BREAKOUT_ART: &[Cel] = &[
    &[
        "rrrrrrrr", "yyyyyyyy", EMPTY, EMPTY, EMPTY, "....w...", "...cc...",
    ],
    &[
        "rrrrrrrr", "yyyyyyyy", EMPTY, EMPTY, "...w....", EMPTY, "...cc...",
    ],
    &[
        "rrrrrrrr", "yyyyyyyy", EMPTY, "..w.....", EMPTY, EMPTY, "...cc...",
    ],
    &[
        "rrrrrrrr", "yyyyyyyy", ".w......", EMPTY, EMPTY, EMPTY, "...cc...",
    ],
    &[
        "rrrrrrrr", "y.yyyyyy", EMPTY, "..w.....", EMPTY, EMPTY, "...cc...",
    ],
    &[
        "rrrrrrrr", "y.yyyyyy", EMPTY, EMPTY, "...w....", EMPTY, "....cc..",
    ],
];

const INVADERS_ART: &[Cel] = &[
    &[
        "..mmmm..", "..cccc..", EMPTY, EMPTY, EMPTY, EMPTY, "...g....",
    ],
    &[
        "..mmmm..", "..cccc..", EMPTY, EMPTY, EMPTY, "...w....", "...g....",
    ],
    &[
        "...mmmm.", "...cccc.", EMPTY, EMPTY, "...w....", EMPTY, "...g....",
    ],
    &[
        "...mmmm.", "...cccc.", EMPTY, "...w....", EMPTY, EMPTY, "...g....",
    ],
    &[
        "...mmmm.", "...cccc.", "...w....", EMPTY, EMPTY, EMPTY, "...g....",
    ],
    &[
        "...mmmm.", "...wccc.", EMPTY, EMPTY, EMPTY, EMPTY, "...g....",
    ],
];

pub const GAMES: &[Game] = &[
    Game {
        name: "Tetris",
        blurb: "stack falling bricks into lines",
        id: "tetris",
        hint: "← → move · ↑ rotate · space drop",
        playable: true,
        art: TETRIS_ART,
    },
    Game {
        name: "Snake",
        blurb: "eat, grow, don't bite yourself",
        id: "snake",
        hint: "← ↑ → ↓ steer",
        playable: false,
        art: SNAKE_ART,
    },
    Game {
        name: "Flappy",
        blurb: "one button between you and the pipes",
        id: "flappy",
        hint: "space to flap",
        playable: false,
        art: FLAPPY_ART,
    },
    Game {
        name: "Pong",
        blurb: "two paddles, one ball, no mercy",
        id: "pong",
        hint: "w/s · ↑/↓ · two players",
        playable: false,
        art: PONG_ART,
    },
    Game {
        name: "Breakout",
        blurb: "angle the bounce, clear the wall",
        id: "breakout",
        hint: "← → paddle",
        playable: false,
        art: BREAKOUT_ART,
    },
    Game {
        name: "Invaders",
        blurb: "shoot the grid before it lands",
        id: "invaders",
        hint: "← → move · space fire",
        playable: false,
        art: INVADERS_ART,
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
