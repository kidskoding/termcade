use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, HighlightSpacing, List, ListItem, Paragraph, Wrap};

use crate::colors::{ACCENT, GO, LOGO_COLORS, SUBTLE, bold, chrome};
use crate::game::{Cel, Game, Row};
use crate::games::GAMES;
use crate::menu::Menu;

const LOGO: &[&str] = &[
    "█████ █████ ██████ █   █ █████  ███  ████  █████",
    "  █   █     █    █ ██ ██ █     █   █ █   █ █    ",
    "  █   █████ ██████ █ █ █ █     █████ █   █ ████ ",
    "  █   █     █   █  █   █ █     █   █ █   █ █    ",
    "  █   █████ █    █ █   █ █████ █   █ ████  █████",
];

const TAGLINE: &str = "a redefined arcade experience for the terminal";

const MIN_W: u16 = 46;
const MIN_H: u16 = 18;
const CEL_MS: u128 = 200;

pub fn fits(area: Rect) -> bool {
    area.width >= MIN_W && area.height >= MIN_H
}

pub fn draw(menu: &mut Menu, frame: &mut Frame) {
    let area = frame.area();
    if !fits(area) {
        let msg = Paragraph::new(format!("needs at least {MIN_W}×{MIN_H}"))
            .style(chrome())
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let blink_on = menu.started.elapsed().as_millis() % 1000 < 500;
    let [_, top, _, tagline, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(LOGO.len() as u16),
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas::<6>(area);

    draw_logo(frame, top);
    draw_tagline(frame, tagline);
    draw_panels(menu, frame, body, blink_on);
    draw_footer(frame, footer);
}

fn draw_logo(frame: &mut Frame, area: Rect) {
    let lines: Vec<Line> = LOGO
        .iter()
        .enumerate()
        .map(|(i, row)| Line::styled(*row, bold(LOGO_COLORS[i % LOGO_COLORS.len()])))
        .collect();

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

fn draw_tagline(frame: &mut Frame, area: Rect) {
    let tagline = Paragraph::new(TAGLINE)
        .style(chrome().add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(tagline, area);
}

fn draw_panels(menu: &mut Menu, frame: &mut Frame, area: Rect, blink_on: bool) {
    let [left, right] =
        Layout::horizontal([Constraint::Length(26), Constraint::Min(1)]).areas::<2>(area);

    draw_game_list(menu, frame, left);
    draw_detail(menu, frame, right, blink_on);

    if let Some(message) = &menu.error {
        draw_error(frame, area, message);
    }
}

fn draw_game_list(menu: &mut Menu, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = GAMES.iter().map(game_item).collect();
    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(bold(GO))
        .block(
            Block::bordered()
                .border_style(chrome())
                .title(Span::styled(" GAMES ", bold(ACCENT))),
        );

    frame.render_stateful_widget(list, area, &mut menu.list_state);
}

fn game_item(game: &Game) -> ListItem<'_> {
    ListItem::new(Line::raw(game.name))
}

fn draw_detail(menu: &mut Menu, frame: &mut Frame, area: Rect, blink_on: bool) {
    let game = &GAMES[menu.selected];

    let block = Block::bordered().border_style(chrome()).title(Span::styled(
        format!(" {} ", game.name.to_uppercase()),
        bold(ACCENT),
    ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![Line::from("")];
    lines.extend(playfield_lines(current_cel(menu, game)));
    lines.push(Line::from(""));
    lines.push(Line::styled(game.name.to_uppercase(), bold(ACCENT)));
    lines.push(Line::styled(game.blurb, Style::default().fg(SUBTLE)));
    lines.push(Line::styled(game.hint, chrome()));
    lines.push(Line::from(""));
    lines.extend(status_lines(blink_on));

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), inner);
}

fn current_cel(menu: &Menu, game: &Game) -> Cel {
    if game.art.is_empty() {
        return &[];
    }

    let elapsed = menu.art_started.elapsed().as_millis();
    game.art[(elapsed / CEL_MS) as usize % game.art.len()]
}

fn playfield_lines(cel: Cel) -> Vec<Line<'static>> {
    let art_w = cel.iter().map(row_width).max().unwrap_or(0);

    let mut lines = vec![Line::styled(format!("╔{}╗", "═".repeat(art_w)), chrome())];
    for row in cel {
        let mut spans = vec![Span::styled("║", chrome())];
        spans.extend(
            row.iter()
                .map(|(text, color)| Span::styled(*text, Style::default().fg(*color))),
        );
        spans.push(Span::styled("║", chrome()));

        lines.push(Line::from(spans));
    }

    lines.push(Line::styled(format!("╚{}╝", "═".repeat(art_w)), chrome()));
    lines
}

fn row_width(row: &Row) -> usize {
    row.iter().map(|(text, _)| text.chars().count()).sum()
}

fn status_lines(blink_on: bool) -> Vec<Line<'static>> {
    if blink_on {
        vec![Line::styled("▶ press enter to play", bold(GO))]
    } else {
        vec![]
    }
}

fn draw_error(frame: &mut Frame, body: Rect, message: &str) {
    let w = body.width.min(46);
    let h = 5;
    let x = body.x + (body.width - w) / 2;
    let y = body.y + body.height.saturating_sub(h) / 2;

    let rect = Rect::new(x, y, w, h);
    frame.render_widget(Clear, rect);

    let dialog = Paragraph::new(Line::styled(
        format!("couldn't launch: {message}"),
        Style::default().fg(Color::Red),
    ))
    .alignment(Alignment::Center)
    .block(
        Block::bordered()
            .title(Span::styled(" ERROR ", bold(Color::Red)))
            .border_style(Style::default().fg(Color::Red)),
    );
    frame.render_widget(dialog, rect);
}

pub fn bars(frame: &mut Frame, covered: u16) {
    let area = frame.area();
    let covered = covered.min(area.height / 2);
    if covered == 0 {
        return;
    }

    for y in [area.y, area.y + area.height - covered] {
        frame.render_widget(Clear, Rect::new(area.x, y, area.width, covered));
    }
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let line = Line::from(vec![
        Span::styled("↑/↓", bold(ACCENT)),
        Span::styled(" navigate", Style::default().fg(SUBTLE)),
        Span::raw("   ·   "),
        Span::styled("enter", bold(GO)),
        Span::styled(" play", Style::default().fg(SUBTLE)),
        Span::raw("   ·   "),
        Span::styled("q", bold(ACCENT)),
        Span::styled(" quit", Style::default().fg(SUBTLE)),
    ]);

    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}
