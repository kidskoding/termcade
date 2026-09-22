use std::collections::HashSet;

use tetris::state::{ActivePiece, Board, HEIGHT, HIDDEN_HEIGHT, PieceKind, WIDTH};

fn empty_board() -> Board {
    Board {
        cells: vec![vec![None; WIDTH]; HEIGHT],
    }
}

const ALL: [PieceKind; 7] = [
    PieceKind::I,
    PieceKind::O,
    PieceKind::T,
    PieceKind::S,
    PieceKind::Z,
    PieceKind::J,
    PieceKind::L,
];

#[test]
fn every_piece_has_four_distinct_cells() {
    for kind in ALL {
        let cells = kind.cells();
        let distinct: HashSet<(i32, i32)> = cells.iter().copied().collect();
        assert_eq!(distinct.len(), 4, "{kind:?} has duplicate cells: {cells:?}");
    }
}

#[test]
fn every_cell_fits_its_bounding_box() {
    for kind in ALL {
        let n = kind.bounding_box() as i32;
        for (r, c) in kind.cells() {
            assert!(
                (0..n).contains(&r) && (0..n).contains(&c),
                "{kind:?} cell ({r},{c}) outside {n}x{n} box"
            );
        }
    }
}

#[test]
fn spawns_centred_and_hidden() {
    for kind in ALL {
        let piece = ActivePiece::spawn(kind);
        let (row, col) = piece.origin;

        assert_eq!(piece.rotation, 0, "{kind:?} does not spawn flat");

        let cols: Vec<i32> = kind.cells().iter().map(|(_, c)| col + c).collect();
        let left = *cols.iter().min().unwrap();
        let right = *cols.iter().max().unwrap();

        assert!(
            left >= 0 && right < WIDTH as i32,
            "{kind:?} spawns outside the board, columns {left}..={right}"
        );

        let slack_left = left;
        let slack_right = WIDTH as i32 - 1 - right;
        assert!(
            (slack_left - slack_right).abs() <= 1,
            "{kind:?} is off centre: {slack_left} empty columns left, {slack_right} right"
        );

        let bottom = kind.cells().iter().map(|(r, _)| row + r).max().unwrap();
        assert!(
            bottom < HIDDEN_HEIGHT as i32,
            "{kind:?} spawns into the visible field at row {bottom}"
        );
    }
}

fn shape(piece: &ActivePiece) -> HashSet<(i32, i32)> {
    piece.blocks().iter().copied().collect()
}

#[test]
fn four_rotations_return_to_the_start() {
    for kind in ALL {
        let start = ActivePiece::spawn(kind);
        let mut piece = start;

        for turn in 1..4 {
            piece.rotate_cw();
            assert_eq!(
                piece.rotation as usize, turn,
                "{kind:?} rotation index did not advance"
            );
        }

        piece.rotate_cw();
        assert_eq!(piece.rotation, 0, "{kind:?} rotation index did not wrap");
        assert_eq!(shape(&piece), shape(&start), "{kind:?} did not return home");
    }
}

#[test]
fn rotating_back_undoes_rotating_forward() {
    for kind in ALL {
        let start = ActivePiece::spawn(kind);
        let mut piece = start;

        piece.rotate_cw();
        piece.rotate_ccw();

        assert_eq!(
            shape(&piece),
            shape(&start),
            "{kind:?} drifted on rotate back"
        );
    }
}

#[test]
fn rotation_stays_inside_the_bounding_box() {
    for kind in ALL {
        let mut piece = ActivePiece::spawn(kind);
        let n = kind.bounding_box() as i32;
        let (row, col) = piece.origin;

        for turn in 0..4 {
            for (r, c) in piece.blocks() {
                assert!(
                    (row..row + n).contains(&r) && (col..col + n).contains(&c),
                    "{kind:?} at rotation {turn} put a cell at ({r},{c}), outside its {n}x{n} box"
                );
            }

            piece.rotate_cw();
        }
    }
}

#[test]
fn only_o_is_unchanged_by_rotation() {
    for kind in ALL {
        let start = ActivePiece::spawn(kind);
        let mut piece = start;
        piece.rotate_cw();

        let unchanged = shape(&piece) == shape(&start);
        assert_eq!(
            unchanged,
            kind == PieceKind::O,
            "{kind:?} rotation changed the shape: {unchanged}"
        );
    }
}

#[test]
fn spawn_is_clear_on_an_empty_board() {
    let board = empty_board();

    for kind in ALL {
        assert!(
            !board.collides(&ActivePiece::spawn(kind)),
            "{kind:?} collides at spawn on an empty board"
        );
    }
}

#[test]
fn walls_and_floor_collide() {
    let board = empty_board();

    for kind in ALL {
        let mut piece = ActivePiece::spawn(kind);
        piece.origin.1 = -(kind.bounding_box() as i32);
        assert!(
            board.collides(&piece),
            "{kind:?} passed through the left wall"
        );

        piece.origin.1 = WIDTH as i32;
        assert!(
            board.collides(&piece),
            "{kind:?} passed through the right wall"
        );

        piece.origin = (HEIGHT as i32, ActivePiece::spawn(kind).origin.1);
        assert!(board.collides(&piece), "{kind:?} passed through the floor");
    }
}

#[test]
fn settled_cells_collide() {
    for kind in ALL {
        let piece = ActivePiece::spawn(kind);
        let mut board = empty_board();
        assert!(!board.collides(&piece));

        let (r, c) = piece.blocks()[0];
        board.cells[r as usize][c as usize] = Some(PieceKind::L);

        assert!(
            board.collides(&piece),
            "{kind:?} overlapped a settled cell at ({r},{c})"
        );
    }
}

#[test]
fn a_piece_falls_until_something_stops_it() {
    let board = empty_board();
    let mut piece = ActivePiece::spawn(PieceKind::T);

    let mut drops = 0;
    loop {
        let mut candidate = piece;
        candidate.origin.0 += 1;

        if board.collides(&candidate) {
            break;
        }

        piece = candidate;
        drops += 1;
        assert!(drops <= HEIGHT, "T never stopped falling");
    }

    let lowest = piece.blocks().iter().map(|(r, _)| *r).max().unwrap();
    assert_eq!(lowest, HEIGHT as i32 - 1, "T did not rest on the floor");
}

#[test]
fn locking_writes_the_piece_into_the_grid() {
    for kind in ALL {
        let mut board = empty_board();
        let piece = ActivePiece::spawn(kind);

        board.lock(&piece);

        let settled = board
            .cells
            .iter()
            .flatten()
            .filter(|cell| cell.is_some())
            .count();
        assert_eq!(settled, 4, "{kind:?} did not settle exactly four cells");

        for (r, c) in piece.blocks() {
            assert_eq!(
                board.cells[r as usize][c as usize],
                Some(kind),
                "{kind:?} missing at ({r},{c})"
            );
        }

        assert!(
            board.collides(&piece),
            "{kind:?} does not block its own cells"
        );
    }
}

#[test]
fn full_rows_clear_and_partial_rows_stay() {
    let mut board = empty_board();
    assert_eq!(board.clear_lines(), 0, "cleared a line on an empty board");

    for c in 0..WIDTH {
        board.cells[HEIGHT - 1][c] = Some(PieceKind::L);
        board.cells[HEIGHT - 2][c] = Some(PieceKind::I);
    }
    board.cells[HEIGHT - 2][0] = None;

    assert_eq!(board.clear_lines(), 1, "did not clear exactly one full row");
    assert_eq!(board.cells.len(), HEIGHT, "board changed height");

    let remaining: Vec<Option<PieceKind>> = board.cells[HEIGHT - 1].clone();
    assert_eq!(
        remaining[0], None,
        "the partial row did not fall into place"
    );
    assert_eq!(remaining[1], Some(PieceKind::I), "wrong row survived");
}

#[test]
fn rows_above_a_clear_fall_by_one() {
    let mut board = empty_board();

    for c in 0..WIDTH {
        board.cells[HEIGHT - 1][c] = Some(PieceKind::L);
    }
    board.cells[HEIGHT - 4][3] = Some(PieceKind::T);

    assert_eq!(board.clear_lines(), 1);
    assert_eq!(
        board.cells[HEIGHT - 3][3],
        Some(PieceKind::T),
        "the floating cell did not drop one row"
    );
}

#[test]
fn a_filled_spawn_area_blocks_every_piece() {
    let mut board = empty_board();
    for row in board.cells.iter_mut().take(HIDDEN_HEIGHT) {
        for cell in row.iter_mut() {
            *cell = Some(PieceKind::L);
        }
    }

    for kind in ALL {
        assert!(
            board.collides(&ActivePiece::spawn(kind)),
            "{kind:?} spawned into a full spawn area without colliding"
        );
    }
}

#[test]
fn the_ghost_rests_on_the_floor_of_an_empty_board() {
    let board = empty_board();

    for kind in ALL {
        let piece = ActivePiece::spawn(kind);
        let ghost = board.landing(&piece);

        assert!(
            !board.collides(&ghost),
            "{kind:?} ghost landed inside something"
        );

        let lowest = ghost.blocks().iter().map(|(r, _)| *r).max().unwrap();
        assert_eq!(
            lowest,
            HEIGHT as i32 - 1,
            "{kind:?} ghost is not on the floor"
        );
    }
}

#[test]
fn the_ghost_sits_on_top_of_the_stack() {
    let mut board = empty_board();
    for c in 0..WIDTH {
        board.cells[HEIGHT - 1][c] = Some(PieceKind::L);
    }

    for kind in ALL {
        let piece = ActivePiece::spawn(kind);
        let ghost = board.landing(&piece);

        let lowest = ghost.blocks().iter().map(|(r, _)| *r).max().unwrap();
        assert_eq!(
            lowest,
            HEIGHT as i32 - 2,
            "{kind:?} ghost did not stop on the stack"
        );
    }
}

#[test]
fn the_ghost_keeps_the_column_and_rotation() {
    let board = empty_board();

    for kind in ALL {
        let mut piece = ActivePiece::spawn(kind);
        piece.rotate_cw();

        let ghost = board.landing(&piece);

        assert_eq!(
            ghost.origin.1, piece.origin.1,
            "{kind:?} ghost drifted sideways"
        );
        assert_eq!(
            ghost.rotation, piece.rotation,
            "{kind:?} ghost changed rotation"
        );
        assert!(
            ghost.origin.0 >= piece.origin.0,
            "{kind:?} ghost rose above the piece"
        );
    }
}

#[test]
fn no_two_pieces_share_a_shape() {
    for (i, a) in ALL.iter().enumerate() {
        for b in &ALL[i + 1..] {
            let sa: HashSet<(i32, i32)> = a.cells().iter().copied().collect();
            let sb: HashSet<(i32, i32)> = b.cells().iter().copied().collect();
            assert_ne!(sa, sb, "{a:?} and {b:?} have identical shapes");
        }
    }
}
