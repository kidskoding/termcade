use crate::game::Cel;

use super::EMPTY;

pub const ART: &[Cel] = &[
    &[EMPTY, EMPTY, EMPTY, ".ggg..r.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "..ggg.r.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "...gggr.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, EMPTY, EMPTY, "....ggg.", EMPTY, EMPTY, EMPTY],
    &[EMPTY, "..r.....", EMPTY, "...gggg.", EMPTY, EMPTY, EMPTY],
    &[
        EMPTY, "..r.....", "......g.", "...ggg..", EMPTY, EMPTY, EMPTY,
    ],
];
