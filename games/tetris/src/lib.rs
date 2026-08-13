pub mod config;
pub mod input;
pub mod render;
pub mod state;

use std::time::{Duration, Instant};

use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::Backend};

use crate::state::{ActivePiece, Board, GameState, PieceKind};

pub fn run<B>(terminal: &mut Terminal<B>) -> color_eyre::Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    let _config = config::load().unwrap_or_default();

    let mut grid = vec![vec![None; state::WIDTH]; state::HEIGHT];
    for c in 0..grid[state::HEIGHT - 1].len() {
        grid[state::HEIGHT - 1][c] = Some(PieceKind::L);
    }

    let active_piece = ActivePiece {
        kind: PieceKind::T,
        rotation: 0,
        origin: (0, 0),
    };
    let board = Board { cells: grid };
    let game_state = GameState {
        board,
        active: active_piece,
        game_over: false,
        drop_timer: Duration::ZERO,
    };

    let tick_interval = Duration::from_millis(16);
    let mut last_tick = Instant::now();
    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_tick);
        let remaining = tick_interval.saturating_sub(elapsed);
        // ponytail: reads process stdin; swap for a per-session channel when SSH lands
        let key = input::poll(remaining);

        terminal.draw(|frame| render::draw(&game_state, frame))?;

        if let Some(k) = key
            && k.code == KeyCode::Char('q')
        {
            break;
        }

        if last_tick.elapsed() >= tick_interval {
            last_tick += tick_interval;
        }
    }

    Ok(())
}
