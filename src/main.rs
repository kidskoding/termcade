use termcade::menu::Menu;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut menu = Menu::new();
    let result = menu.run(&mut terminal);

    ratatui::restore();
    result
}
