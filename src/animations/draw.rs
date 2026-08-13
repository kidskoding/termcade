use std::time::Duration;

use ratatui::prelude::*;

use crate::colors::chrome;
use crate::game::Cel;

const CEL_MS: u128 = 200;

pub const TILE_W: usize = 6;

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
        '.' => None,
        _ => Some(Color::Red),
    }
}

pub fn tile(mark: char) -> Option<(&'static str, Color, Color)> {
    const DARK: Color = Color::Rgb(0x77, 0x6e, 0x65);
    const LIGHT: Color = Color::Rgb(0xf9, 0xf6, 0xf2);

    let (label, bg, fg) = match mark {
        '.' => ("      ", Color::Rgb(0xcd, 0xc1, 0xb4), DARK),
        '1' => ("  2   ", Color::Rgb(0xee, 0xe4, 0xda), DARK),
        '2' => ("  4   ", Color::Rgb(0xed, 0xe0, 0xc8), DARK),
        '3' => ("  8   ", Color::Rgb(0xf2, 0xb1, 0x79), LIGHT),
        '4' => ("  16  ", Color::Rgb(0xf5, 0x95, 0x63), LIGHT),
        '5' => ("  32  ", Color::Rgb(0xf6, 0x7c, 0x5f), LIGHT),
        '6' => ("  64  ", Color::Rgb(0xf6, 0x5e, 0x3b), LIGHT),
        '7' => (" 128  ", Color::Rgb(0xed, 0xcf, 0x72), LIGHT),
        '8' => (" 256  ", Color::Rgb(0xed, 0xcc, 0x61), LIGHT),
        '9' => (" 512  ", Color::Rgb(0xed, 0xc8, 0x50), LIGHT),
        'a' => (" 1024 ", Color::Rgb(0xed, 0xc5, 0x3f), LIGHT),
        'b' => (" 2048 ", Color::Rgb(0xed, 0xc2, 0x2e), LIGHT),
        _ => return None,
    };

    Some((label, bg, fg))
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

pub fn tiles(cel: Cel) -> Vec<Line<'static>> {
    let art_w = cel.iter().map(|row| row.chars().count()).max().unwrap_or(0) * TILE_W;

    framed(art_w, |lines| {
        for row in cel {
            for numbered in [false, true, false] {
                let mut spans = vec![Span::styled("║", chrome())];
                spans.extend(row.chars().map(|mark| {
                    let (label, bg, fg) = tile(mark).unwrap_or(("      ", Color::Red, Color::Red));
                    let text = if numbered { label } else { "      " };
                    Span::styled(text, Style::default().fg(fg).bg(bg))
                }));
                spans.push(Span::styled("║", chrome()));

                lines.push(Line::from(spans));
            }
        }
    })
}

fn framed(art_w: usize, fill: impl FnOnce(&mut Vec<Line<'static>>)) -> Vec<Line<'static>> {
    let mut lines = vec![Line::styled(format!("╔{}╗", "═".repeat(art_w)), chrome())];
    fill(&mut lines);
    lines.push(Line::styled(format!("╚{}╝", "═".repeat(art_w)), chrome()));

    lines
}
