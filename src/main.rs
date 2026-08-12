mod games;
mod menu;
mod render;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = menu::App::new();
    let result = app.run(&mut terminal);

    ratatui::restore();
    result
}
