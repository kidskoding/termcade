use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
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
