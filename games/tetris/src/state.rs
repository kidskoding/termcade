use std::time::Duration;

pub const WIDTH: usize = 10;
pub const HIDDEN_HEIGHT: usize = 2;
pub const VISIBLE_HEIGHT: usize = 20;
pub const HEIGHT: usize = HIDDEN_HEIGHT + VISIBLE_HEIGHT;

pub struct Board {
    pub cells: Vec<Vec<Option<PieceKind>>>,
}

impl Board {
    pub fn lock(&mut self, piece: &ActivePiece) {
        for (r, c) in piece.blocks() {
            if (0..HEIGHT as i32).contains(&r) && (0..WIDTH as i32).contains(&c) {
                self.cells[r as usize][c as usize] = Some(piece.kind);
            }
        }
    }

    pub fn clear_lines(&mut self) -> usize {
        let before = self.cells.len();
        self.cells.retain(|row| row.iter().any(Option::is_none));

        let cleared = before - self.cells.len();
        for _ in 0..cleared {
            self.cells.insert(0, vec![None; WIDTH]);
        }

        cleared
    }

    pub fn landing(&self, piece: &ActivePiece) -> ActivePiece {
        let mut resting = *piece;

        loop {
            let mut candidate = resting;
            candidate.origin.0 += 1;

            if self.collides(&candidate) {
                return resting;
            }

            resting = candidate;
        }
    }

    pub fn collides(&self, piece: &ActivePiece) -> bool {
        piece.blocks().iter().any(|&(r, c)| {
            r < 0
                || c < 0
                || r >= HEIGHT as i32
                || c >= WIDTH as i32
                || self.cells[r as usize][c as usize].is_some()
        })
    }
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

#[derive(Clone, Copy)]
pub struct ActivePiece {
    pub kind: PieceKind,
    pub rotation: u8,
    pub origin: (i32, i32),
}

impl ActivePiece {
    pub fn spawn(kind: PieceKind) -> Self {
        let col = (WIDTH - kind.bounding_box()) / 2;

        Self {
            kind,
            rotation: 0,
            origin: (0, col as i32),
        }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }

    pub fn blocks(&self) -> [(i32, i32); 4] {
        let n = self.kind.bounding_box() as i32;
        let (row, col) = self.origin;
        let mut cells = self.kind.cells();

        for _ in 0..self.rotation % 4 {
            for (r, c) in &mut cells {
                (*r, *c) = (*c, n - 1 - *r);
            }
        }

        cells.map(|(r, c)| (row + r, col + c))
    }
}
