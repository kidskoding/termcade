use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
    &[
        EMPTY, EMPTY, "bbbbbbbb", "r.y.wwww", "bbbbbbbb", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "bbbbbbbb", ".r.y.www", "bbbbbbbb", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "bbbbbbbb", "..r.y.ww", "bbbbbbbb", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "bbbbbbbb", "...r.y.w", "bbbbbbbb", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "bbbbbbbb", "....r.y.", "bbbbbbbb", EMPTY, EMPTY,
    ],
    &[
        EMPTY, EMPTY, "bbbbbbbb", "....c.y.", "bbbbbbbb", EMPTY, EMPTY,
    ],
];
