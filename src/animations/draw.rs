use std::time::Duration;

use ratatui::prelude::*;

use crate::colors::chrome;
use crate::game::Cel;

const CEL_MS: u128 = 200;

pub fn cel_at(art: &'static [Cel], elapsed: Duration) -> Cel {
    if art.is_empty() {
        return &[];
    }

    art[(elapsed.as_millis() / CEL_MS) as usize % art.len()]
}

pub fn cel_color(mark: char) -> Option<Color> {
    match mark {
        'r' => Some(Color::Red),
        'y' => Some(Color::Yellow),
        'g' => Some(Color::Green),
        'c' => Some(Color::Cyan),
        'b' => Some(Color::Blue),
        'm' => Some(Color::Magenta),
        'w' => Some(Color::White),
        '0' => Some(Color::Rgb(0xcd, 0xc1, 0xb4)),
        '1' => Some(Color::Rgb(0xee, 0xe4, 0xda)),
        '2' => Some(Color::Rgb(0xed, 0xe0, 0xc8)),
        '3' => Some(Color::Rgb(0xf2, 0xb1, 0x79)),
        '4' => Some(Color::Rgb(0xf5, 0x95, 0x63)),
        '5' => Some(Color::Rgb(0xf6, 0x7c, 0x5f)),
        '6' => Some(Color::Rgb(0xf6, 0x5e, 0x3b)),
        '7' => Some(Color::Rgb(0xed, 0xcf, 0x72)),
        '8' => Some(Color::Rgb(0xed, 0xcc, 0x61)),
        '9' => Some(Color::Rgb(0xed, 0xc8, 0x50)),
        'A' => Some(Color::Rgb(0xed, 0xc5, 0x3f)),
        'B' => Some(Color::Rgb(0xed, 0xc2, 0x2e)),
        '.' => None,
        _ => Some(Color::Red),
    }
}

pub fn blocks(cel: Cel) -> Vec<Line<'static>> {
    let art_w = cel.iter().map(|row| row.chars().count()).max().unwrap_or(0) * 2;

    framed(art_w, |lines| {
        for row in cel {
            let mut spans = vec![Span::styled("║", chrome())];
            spans.extend(row.chars().map(|mark| match cel_color(mark) {
                Some(color) => Span::styled("██", Style::default().fg(color)),
                None => Span::raw("  "),
            }));
            spans.push(Span::styled("║", chrome()));

            lines.push(Line::from(spans));
        }
    })
}

fn framed(art_w: usize, fill: impl FnOnce(&mut Vec<Line<'static>>)) -> Vec<Line<'static>> {
    let mut lines = vec![Line::styled(format!("╔{}╗", "═".repeat(art_w)), chrome())];
    fill(&mut lines);
    lines.push(Line::styled(format!("╚{}╝", "═".repeat(art_w)), chrome()));

    lines
}
