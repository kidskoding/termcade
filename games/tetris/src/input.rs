use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub fn poll(timeout: Duration) -> Option<KeyEvent> {
    if !event::poll(timeout).ok()? {
        return None;
    }

    let evt = event::read().ok()?;
    match evt {
        Event::Key(k) => Some(k),
        _ => None,
    }
}
