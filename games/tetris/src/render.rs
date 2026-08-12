use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};

use crate::state::{GameState, HEIGHT, HIDDEN_HEIGHT, PieceKind, VISIBLE_HEIGHT, WIDTH};

pub fn draw(state: &GameState, frame: &mut Frame) {
    let area = frame.area();
    const MIN_WIDTH: usize = 24;
    const MIN_HEIGHT: usize = 30;

    if area.width < MIN_WIDTH as u16 || area.height < MIN_HEIGHT as u16 {
        let h: u16 = 3;
        let y = area.y + area.height.saturating_sub(h) / 2;
        let rect = Rect::new(area.x, y, area.width, h);

        let paragraph = Paragraph::new("terminal dimensions required: 24x30!")
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, rect);
        return;
    }

    let mut vec_outer: Vec<Line> = vec![];
    for r in HIDDEN_HEIGHT..HEIGHT {
        let mut vec_inner: Vec<Span> = vec![];
        for c in 0..WIDTH {
            let cell = state.board.cells[r][c];
            let span = match cell {
                Some(kind) => Span::styled("██", Style::default().fg(get_color(kind))),
                None => Span::raw("  "),
            };

            vec_inner.push(span);
        }
        vec_outer.push(Line::from(vec_inner))
    }

    let rect = Rect::new(
        area.x + (area.width - WIDTH as u16 * 2) / 2,
        area.y + (area.height - VISIBLE_HEIGHT as u16) / 2,
        WIDTH as u16 * 2,
        VISIBLE_HEIGHT as u16,
    );

    frame.render_widget(Paragraph::new(vec_outer), rect);
}

pub fn get_color(kind: PieceKind) -> Color {
    match kind {
        PieceKind::I => Color::Cyan,
        PieceKind::O => Color::Yellow,
        PieceKind::T => Color::Magenta,
        PieceKind::S => Color::Green,
        PieceKind::Z => Color::Red,
        PieceKind::J => Color::Blue,
        PieceKind::L => Color::Rgb(255, 196, 0),
    }
}
