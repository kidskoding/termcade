use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use crate::games::{self, GAMES, Game};
use crate::menu::App;

pub fn ui(app: &mut App, frame: &mut Frame) {
    let area = frame.area();

    let items: Vec<ListItem> = GAMES.iter().map(game_item).collect();
    let list = List::new(items).highlight_symbol("▶ ").highlight_style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );

    frame.render_stateful_widget(list, area, &mut app.list_state);

    if let Some(message) = &app.error {
        draw_error(frame, area, message);
    }
}

fn game_item(game: &Game) -> ListItem<'_> {
    let built = games::available(game);

    let text = if built {
        game.name.to_string()
    } else {
        format!("{} (not built)", game.name)
    };

    ListItem::new(text)
}

fn draw_error(frame: &mut Frame, area: Rect, message: &str) {
    let w = area.width.min(46);
    let h = 3;
    let x = area.x + (area.width - w) / 2;
    let y = area.y + area.height.saturating_sub(h) / 2;

    let rect = Rect::new(x, y, w, h);

    let dialog = Paragraph::new(format!("couldn't launch: {message}"))
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center)
        .block(Block::bordered().title(" ERROR "));

    frame.render_widget(dialog, rect);
}
