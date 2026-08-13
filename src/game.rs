use ratatui::prelude::Color;

pub type Seg = (&'static str, Color);
pub type Row = &'static [Seg];
pub type Cel = &'static [Row];

pub struct Game {
    pub name: &'static str,
    pub blurb: &'static str,
    pub id: &'static str,
    pub hint: &'static str,
    pub art: &'static [Cel],
}
