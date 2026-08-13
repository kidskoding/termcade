use ratatui::prelude::Color;

pub type Seg = (&'static str, Color);
pub type Row = &'static [Seg];

/// One frame of a cabinet's looping art. Every cel of a game must be the same
/// size or the detail panel jitters — `tests/cabinets.rs` enforces it.
pub type Cel = &'static [Row];

pub struct Game {
    pub name: &'static str,
    pub blurb: &'static str,
    pub id: &'static str,
    pub hint: &'static str,
    pub art: &'static [Cel],
}
