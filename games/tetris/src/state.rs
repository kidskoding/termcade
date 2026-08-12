use std::time::Duration;

pub const WIDTH: usize = 10;
pub const HIDDEN_HEIGHT: usize = 2;
pub const VISIBLE_HEIGHT: usize = 20;
pub const HEIGHT: usize = HIDDEN_HEIGHT + VISIBLE_HEIGHT;

pub struct Board {
    pub cells: Vec<Vec<Option<PieceKind>>>,
}

pub struct GameState {
    pub board: Board,
    pub active: ActivePiece,
    pub game_over: bool,
    pub drop_timer: Duration,
}

// the seven variants of the tetronimo
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}
impl PieceKind {
    pub fn bounding_box(&self) -> usize {
        match self {
            PieceKind::I => 4,
            PieceKind::O => 2,
            PieceKind::T | PieceKind::S | PieceKind::Z | PieceKind::J | PieceKind::L => 3,
        }
    }

    pub fn cells(&self) -> [(i32, i32); 4] {
        match self {
            PieceKind::T => [(1, 0), (1, 1), (1, 2), (0, 1)],
            PieceKind::Z => [(0, 0), (0, 1), (1, 1), (1, 2)],
            PieceKind::S => [(1, 0), (0, 1), (0, 2), (1, 1)],
            PieceKind::I => [(1, 0), (1, 1), (1, 2), (1, 3)],
            PieceKind::O => [(0, 0), (0, 1), (1, 0), (1, 1)],
            PieceKind::J => [(0, 0), (1, 0), (1, 1), (1, 2)],
            PieceKind::L => [(1, 0), (1, 1), (1, 2), (0, 2)],
        }
    }
}

pub struct ActivePiece {
    pub kind: PieceKind,
    pub rotation: u8,
    pub origin: (i32, i32),
}
