use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
    &[
        "..mmmm..", "..cccc..", EMPTY, EMPTY, EMPTY, EMPTY, "...g....",
    ],
    &[
        "..mmmm..", "..c.cc..", "....y...", EMPTY, EMPTY, EMPTY, "...g....",
    ],
    &[
        "..mmmm..", "..c.cc..", EMPTY, ".....y..", EMPTY, "...w....", "...g....",
    ],
    &[
        "..mmmm..", "..c.cc..", EMPTY, EMPTY, "...wy...", EMPTY, "...g....",
    ],
    &[
        "..mmmm..", "..c.cc..", EMPTY, EMPTY, "...ww...", EMPTY, "...g....",
    ],
    &[
        "..mmmm..", "..c.cc..", EMPTY, EMPTY, EMPTY, EMPTY, "...g....",
    ],
];
