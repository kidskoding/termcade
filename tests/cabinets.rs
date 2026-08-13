use termcade::game::Row;
use termcade::games::GAMES;

fn row_width(row: &Row) -> usize {
    row.iter().map(|(text, _)| text.chars().count()).sum()
}

#[test]
fn cabinet_cels_are_uniform() {
    for game in GAMES {
        let first = game
            .art
            .first()
            .unwrap_or_else(|| panic!("{}: no art cels", game.name));

        let rows = first.len();
        let width = first.first().map(row_width).unwrap_or(0);

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
                    row_width(row),
                    width,
                    "{}: cel {c} row {r} is {} wide, expected {width}",
                    game.name,
                    row_width(row)
                );
            }
        }
    }
}
