use ratatui::prelude::{Color, Modifier, Style};

pub const ACCENT: Color = Color::LightRed;
pub const GO: Color = Color::Green;
pub const CHROME: Color = Color::DarkGray;
pub const SUBTLE: Color = Color::Gray;

pub const LOGO_COLORS: [Color; 4] = [Color::Red, Color::LightRed, Color::Yellow, Color::LightRed];

pub fn bold(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

pub fn chrome() -> Style {
    Style::default().fg(CHROME)
}
