use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
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
