use ratatui::prelude::Color;

pub struct Game {
    pub name: &'static str,
    pub blurb: &'static str,
    pub bin: &'static str,
    pub hint: &'static str,
    pub art: &'static [(&'static str, Color)],
}
