use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::ListState;

use crate::games::{self, GAMES};
use crate::render;
use crate::transition::{self, Wipe};

pub struct Menu {
    pub selected: usize,
    pub list_state: ListState,
    pub error: Option<String>,
    pub started: Instant,
    pub art_started: Instant,
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
        self.art_started = Instant::now();
    }

    fn launch_selected(
        &mut self,
        terminal: &mut ratatui::DefaultTerminal,
    ) -> color_eyre::Result<()> {
        if !GAMES[self.selected].playable {
            return Ok(());
        }

        transition::wipe(self, terminal, Wipe::Closing)?;

        let game = &GAMES[self.selected];
        self.error = games::launch(game, terminal).err().map(|e| e.to_string());

        terminal.clear()?;
        transition::wipe(self, terminal, Wipe::Opening)?;

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
            art_started: Instant::now(),
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
