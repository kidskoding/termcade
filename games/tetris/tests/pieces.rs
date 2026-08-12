use std::collections::HashSet;

use tetrotui::state::PieceKind;

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
fn no_two_pieces_share_a_shape() {
    for (i, a) in ALL.iter().enumerate() {
        for b in &ALL[i + 1..] {
            let sa: HashSet<(i32, i32)> = a.cells().iter().copied().collect();
            let sb: HashSet<(i32, i32)> = b.cells().iter().copied().collect();
            assert_ne!(sa, sb, "{a:?} and {b:?} have identical shapes");
        }
    }
}
