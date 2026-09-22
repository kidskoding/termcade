use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Wrap},
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

    let blocks = state.active.blocks();
    let ghost = state.board.landing(&state.active).blocks();
    let active = get_color(state.active.kind);

    let mut vec_outer: Vec<Line> = vec![];
    for r in HIDDEN_HEIGHT..HEIGHT {
        let mut vec_inner: Vec<Span> = vec![];
        for c in 0..WIDTH {
            let at = (r as i32, c as i32);

            let span = if let Some(kind) = state.board.cells[r][c] {
                Span::styled("██", Style::default().fg(get_color(kind)))
            } else if blocks.contains(&at) {
                Span::styled("██", Style::default().fg(active))
            } else if ghost.contains(&at) {
                Span::styled(
                    "░░",
                    Style::default().fg(active).add_modifier(Modifier::DIM),
                )
            } else {
                Span::raw("  ")
            };

            vec_inner.push(span);
        }
        vec_outer.push(Line::from(vec_inner))
    }

    let width = WIDTH as u16 * 2 + 2;
    let height = VISIBLE_HEIGHT as u16 + 2;
    let rect = Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    );

    let block = Block::bordered().border_style(Style::default().fg(Color::DarkGray));
    frame.render_widget(Paragraph::new(vec_outer).block(block), rect);

    if state.game_over {
        let banner = Rect::new(rect.x + 1, rect.y + height / 2 - 1, width - 2, 3);
        frame.render_widget(Clear, banner);
        frame.render_widget(
            Paragraph::new("GAME OVER")
                .alignment(Alignment::Center)
                .block(Block::bordered().border_style(Style::default().fg(Color::Red))),
            banner,
        );
    }
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
