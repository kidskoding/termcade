use termcade::games::GAMES;
use termcade::render::{TILE_W, cel_color, tile};

#[test]
fn cabinet_cels_are_uniform() {
    for game in GAMES {
        let first = game
            .art
            .first()
            .unwrap_or_else(|| panic!("{}: no art cels", game.name));

        let rows = first.len();
        let width = first.first().map(|row| row.chars().count()).unwrap_or(0);

        for (c, cel) in game.art.iter().enumerate() {
            assert_eq!(
                cel.len(),
                rows,
                "{}: cel {c} has {} rows, expected {rows}",
                game.name,
                cel.len()
            );

            for (r, row) in cel.iter().enumerate() {
                assert_eq!(
                    row.chars().count(),
                    width,
                    "{}: cel {c} row {r} is {:?}, expected {width} cells",
                    game.name,
                    row
                );
            }
        }
    }
}

#[test]
fn cabinet_art_marks_are_known() {
    for game in GAMES {
        for (c, cel) in game.art.iter().enumerate() {
            for (r, row) in cel.iter().enumerate() {
                for mark in row.chars() {
                    if game.tiles {
                        assert!(
                            tile(mark).is_some(),
                            "{}: cel {c} row {r} has unknown tile mark {mark:?}",
                            game.name
                        );
                    } else {
                        assert!(
                            matches!(mark, '.' | 'r' | 'y' | 'g' | 'c' | 'b' | 'm' | 'w'),
                            "{}: cel {c} row {r} has unknown mark {mark:?}",
                            game.name
                        );
                        assert_eq!(cel_color(mark).is_none(), mark == '.');
                    }
                }
            }
        }
    }
}

#[test]
fn tile_labels_fill_the_tile() {
    for mark in ".123456789ab".chars() {
        let (label, _, _) = tile(mark).unwrap();
        assert_eq!(
            label.chars().count(),
            TILE_W,
            "tile {mark:?} label {label:?} is not {TILE_W} wide"
        );
    }
}

#[test]
fn tile_numbers_are_centered() {
    for mark in "123456789ab".chars() {
        let (label, _, _) = tile(mark).unwrap();
        let digits = label.trim().chars().count();
        let pad = label.chars().take_while(|c| *c == ' ').count();

        assert_eq!(
            pad,
            (TILE_W - digits) / 2,
            "tile {mark:?} label {label:?} is not centered with the odd space on the right"
        );
    }
}
