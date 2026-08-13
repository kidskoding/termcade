use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;

use crate::games::{self, GAMES};
use crate::render;

const WIPE_CLOSE: Duration = Duration::from_millis(150);
const WIPE_OPEN: Duration = Duration::from_millis(120);
const WIPE_STEPS: u16 = 12;

#[derive(Clone, Copy)]
enum Wipe {
    Closing,
    Opening,
}

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

        self.wipe(terminal, Wipe::Closing)?;

        let game = &GAMES[self.selected];
        self.error = games::launch(game, terminal).err().map(|e| e.to_string());

        terminal.clear()?;
        self.wipe(terminal, Wipe::Opening)?;

        Ok(())
    }

    fn wipe(
        &mut self,
        terminal: &mut ratatui::DefaultTerminal,
        wipe: Wipe,
    ) -> color_eyre::Result<()> {
        let size = terminal.size()?;
        if !render::fits(Rect::new(0, 0, size.width, size.height)) {
            return Ok(());
        }

        let half = size.height / 2;
        let steps = half.min(WIPE_STEPS);
        if steps == 0 {
            return Ok(());
        }

        let total = match wipe {
            Wipe::Closing => WIPE_CLOSE,
            Wipe::Opening => WIPE_OPEN,
        };
        let per_step = total / u32::from(steps);
        for step in 0..=steps {
            let covered = match wipe {
                Wipe::Closing => step,
                Wipe::Opening => steps - step,
            };

            terminal.draw(|frame| {
                render::draw(self, frame);
                render::bars(frame, covered * half / steps);
            })?;

            thread::sleep(per_step);
        }

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
