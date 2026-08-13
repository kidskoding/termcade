use termcade::animations::cel_color;
use termcade::games::GAMES;

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
fn every_cabinet_is_the_same_size() {
    let size = |game: &termcade::game::Game| {
        let cel = game.art.first().expect("no art cels");
        (cel.len(), cel.first().map(|row| row.chars().count()))
    };

    let first = &GAMES[0];
    for game in GAMES {
        assert_eq!(
            size(game),
            size(first),
            "{} does not match {}'s preview size",
            game.name,
            first.name
        );
    }
}

#[test]
fn cabinet_art_marks_are_known() {
    for game in GAMES {
        for (c, cel) in game.art.iter().enumerate() {
            for (r, row) in cel.iter().enumerate() {
                for mark in row.chars() {
                    assert!(
                        matches!(
                            mark,
                            '.' | '0'..='9' | 'A' | 'B' | 'r' | 'y' | 'g' | 'c' | 'b' | 'm' | 'w'
                        ),
                        "{}: cel {c} row {r} has unknown mark {mark:?}",
                        game.name
                    );
                    assert_eq!(cel_color(mark).is_none(), mark == '.');
                }
            }
        }
    }
}
