fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let result = tetris::run(&mut terminal);

    ratatui::restore();
    result
}
