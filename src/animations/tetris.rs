use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
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
