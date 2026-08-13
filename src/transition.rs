use std::thread;
use std::time::Duration;

use ratatui::prelude::*;
use ratatui::widgets::Clear;

use crate::menu::Menu;
use crate::render;

const CLOSE: Duration = Duration::from_millis(150);
const OPEN: Duration = Duration::from_millis(120);
const STEPS: u16 = 12;

#[derive(Clone, Copy)]
pub enum Wipe {
    Closing,
    Opening,
}

pub fn wipe(
    menu: &mut Menu,
    terminal: &mut ratatui::DefaultTerminal,
    wipe: Wipe,
) -> color_eyre::Result<()> {
    let size = terminal.size()?;
    if !render::fits(Rect::new(0, 0, size.width, size.height)) {
        return Ok(());
    }

    let half = size.height / 2;
    let steps = half.min(STEPS);
    if steps == 0 {
        return Ok(());
    }

    let total = match wipe {
        Wipe::Closing => CLOSE,
        Wipe::Opening => OPEN,
    };
    let per_step = total / u32::from(steps);

    for step in 0..=steps {
        let covered = match wipe {
            Wipe::Closing => step,
            Wipe::Opening => steps - step,
        };

        terminal.draw(|frame| {
            render::draw(menu, frame);
            bars(frame, covered * half / steps);
        })?;

        thread::sleep(per_step);
    }

    Ok(())
}

fn bars(frame: &mut Frame, covered: u16) {
    let area = frame.area();
    let covered = covered.min(area.height / 2);
    if covered == 0 {
        return;
    }

    for y in [area.y, area.y + area.height - covered] {
        frame.render_widget(Clear, Rect::new(area.x, y, area.width, covered));
    }
}
