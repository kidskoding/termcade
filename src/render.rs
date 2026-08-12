use ratatui::prelude::*;
use ratatui::widgets::{Block, Clear, HighlightSpacing, List, ListItem, Paragraph, Wrap};

use crate::games::{self, GAMES, Game};
use crate::menu::App;

const ACCENT: Color = Color::LightRed;
const GO: Color = Color::Green;
const CHROME: Color = Color::DarkGray;
const SUBTLE: Color = Color::Gray;

const LOGO_COLORS: [Color; 4] = [Color::Red, Color::LightRed, Color::Yellow, Color::LightRed];

fn bold(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::BOLD)
}

fn chrome() -> Style {
    Style::default().fg(CHROME)
}

const LOGO: &[&str] = &[
    "█████ █████ ██████ █   █ █████  ███  ████  █████",
    "  █   █     █    █ ██ ██ █     █   █ █   █ █    ",
    "  █   █████ ██████ █ █ █ █     █████ █   █ ████ ",
    "  █   █     █   █  █   █ █     █   █ █   █ █    ",
    "  █   █████ █    █ █   █ █████ █   █ ████  █████",
];

const TAGLINE: &str =
    "a collection of terminal arcade games to play when you are bored during that 8 am lecture";

const MIN_W: u16 = 46;
const MIN_H: u16 = 18;

pub fn ui(app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    if area.width < MIN_W || area.height < MIN_H {
        let msg = Paragraph::new(format!("needs at least {MIN_W}×{MIN_H}"))
            .style(chrome())
            .alignment(Alignment::Center);
        frame.render_widget(msg, area);
        return;
    }

    let blink_on = app.started.elapsed().as_millis() % 1000 < 500;
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
    draw_panels(app, frame, body, blink_on);
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

fn draw_panels(app: &mut App, frame: &mut Frame, area: Rect, blink_on: bool) {
    let [left, right] =
        Layout::horizontal([Constraint::Length(26), Constraint::Min(1)]).areas::<2>(area);

    draw_game_list(app, frame, left);
    draw_detail(app, frame, right, blink_on);

    if let Some(message) = &app.error {
        draw_error(frame, area, message);
    }
}

fn draw_game_list(app: &mut App, frame: &mut Frame, area: Rect) {
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

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn game_item(game: &Game) -> ListItem<'_> {
    let built = games::available(game);

    let text = if built {
        game.name.to_string()
    } else {
        format!("{} (not built)", game.name)
    };

    let style = if built { Style::default() } else { chrome() };

    ListItem::new(Line::styled(text, style))
}

fn draw_detail(app: &mut App, frame: &mut Frame, area: Rect, blink_on: bool) {
    let game = &GAMES[app.selected];
    let built = games::available(game);
    let main_color = if built { ACCENT } else { CHROME };

    let block = Block::bordered().border_style(chrome()).title(Span::styled(
        format!(" {} ", game.name.to_uppercase()),
        bold(main_color),
    ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = vec![Line::from("")];
    lines.extend(playfield_lines(game, built));
    lines.push(Line::from(""));
    lines.push(Line::styled(game.name.to_uppercase(), bold(main_color)));
    lines.push(Line::styled(game.blurb, Style::default().fg(SUBTLE)));
    lines.push(Line::styled(game.hint, chrome()));
    lines.push(Line::from(""));
    lines.extend(status_lines(game, built, blink_on));

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), inner);
}

fn playfield_lines(game: &Game, built: bool) -> Vec<Line<'_>> {
    let art_w = game
        .art
        .iter()
        .map(|(row, _)| row.chars().count())
        .max()
        .unwrap_or(0);

    let mut lines = vec![Line::styled(format!("╔{}╗", "═".repeat(art_w)), chrome())];
    for (row, color) in game.art {
        let piece_style = if built {
            Style::default().fg(*color)
        } else {
            chrome()
        };

        lines.push(Line::from(vec![
            Span::styled("║", chrome()),
            Span::styled(*row, piece_style),
            Span::styled("║", chrome()),
        ]));
    }

    lines.push(Line::styled(format!("╚{}╝", "═".repeat(art_w)), chrome()));
    lines
}

fn status_lines(game: &Game, built: bool, blink_on: bool) -> Vec<Line<'_>> {
    if built {
        if blink_on {
            vec![Line::styled("▶ press enter to play", bold(GO))]
        } else {
            vec![]
        }
    } else {
        vec![
            Line::styled("not installed", bold(Color::Red)),
            Line::styled(format!("cargo build -p {}", game.bin), chrome()),
        ]
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
