use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

use crate::games::{self, GAMES};
use crate::render;

pub struct Menu {
    pub selected: usize,
    pub list_state: ListState,
    pub error: Option<String>,
    pub started: Instant,
}

impl Menu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| render::draw(self, frame))?;

            if let Some(code) = read_key()? {
                match code {
                    KeyCode::Up => self.select((self.selected + GAMES.len() - 1) % GAMES.len()),
                    KeyCode::Down => self.select((self.selected + 1) % GAMES.len()),
                    KeyCode::Enter => self.launch_selected(terminal)?,
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }

    fn select(&mut self, index: usize) {
        self.selected = index;
        self.list_state.select(Some(index));
    }

    fn launch_selected(
        &mut self,
        terminal: &mut ratatui::DefaultTerminal,
    ) -> color_eyre::Result<()> {
        let game = &GAMES[self.selected];

        ratatui::restore();
        self.error = games::launch(game).err().map(|e| e.to_string());
        *terminal = ratatui::init();

        Ok(())
    }
}

impl Default for Menu {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            selected: 0,
            list_state,
            error: None,
            started: Instant::now(),
        }
    }
}

fn read_key() -> color_eyre::Result<Option<KeyCode>> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(None);
    }

    if let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(Some(key.code));
    }

    Ok(None)
}
