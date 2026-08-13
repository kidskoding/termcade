use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
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
