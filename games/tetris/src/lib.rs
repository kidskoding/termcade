pub mod config;
pub mod input;
pub mod render;
pub mod state;

use std::time::{Duration, Instant};

use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::Backend};

use crate::state::{ActivePiece, Board, GameState, PieceKind};

const GRAVITY: Duration = Duration::from_millis(500);

const ORDER: [PieceKind; 7] = [
    PieceKind::I,
    PieceKind::O,
    PieceKind::T,
    PieceKind::S,
    PieceKind::Z,
    PieceKind::J,
    PieceKind::L,
];

pub fn run<B>(terminal: &mut Terminal<B>) -> color_eyre::Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    let _config = config::load().unwrap_or_default();

    let mut next = 0;
    let active_piece = ActivePiece::spawn(ORDER[next]);
    let board = Board {
        cells: vec![vec![None; state::WIDTH]; state::HEIGHT],
    };
    let mut game_state = GameState {
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
        let key = input::poll(remaining);

        terminal.draw(|frame| render::draw(&game_state, frame))?;

        if let Some(k) = key {
            match k.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                _ if game_state.game_over => {}
                KeyCode::Left => {
                    shift(&mut game_state, 0, -1);
                }
                KeyCode::Right => {
                    shift(&mut game_state, 0, 1);
                }
                KeyCode::Down => {
                    if shift(&mut game_state, 1, 0) {
                        game_state.drop_timer = Duration::ZERO;
                    }
                }
                KeyCode::Up => rotate(&mut game_state),
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_interval {
            last_tick += tick_interval;

            if !game_state.game_over {
                gravity(&mut game_state, &mut next, tick_interval);
            }
        }
    }

    Ok(())
}

fn shift(state: &mut GameState, rows: i32, cols: i32) -> bool {
    let mut candidate = state.active;
    candidate.origin.0 += rows;
    candidate.origin.1 += cols;

    if state.board.collides(&candidate) {
        return false;
    }

    state.active = candidate;
    true
}

fn rotate(state: &mut GameState) {
    let mut candidate = state.active;
    candidate.rotate_cw();

    if !state.board.collides(&candidate) {
        state.active = candidate;
    }
}

fn gravity(state: &mut GameState, next: &mut usize, tick: Duration) {
    state.drop_timer += tick;
    if state.drop_timer < GRAVITY {
        return;
    }

    state.drop_timer = Duration::ZERO;
    if shift(state, 1, 0) {
        return;
    }

    state.board.lock(&state.active);
    state.board.clear_lines();

    *next = (*next + 1) % ORDER.len();
    let spawned = ActivePiece::spawn(ORDER[*next]);

    if state.board.collides(&spawned) {
        state.game_over = true;
    } else {
        state.active = spawned;
    }
}
