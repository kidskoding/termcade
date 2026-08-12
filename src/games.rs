use std::path::PathBuf;
use std::process::Command;

use color_eyre::eyre::{Result, bail};

pub struct Game {
    pub name: &'static str,
    pub bin: &'static str,
}

pub const GAMES: &[Game] = &[Game {
    name: "Tetris",
    bin: "tetris",
}];

pub fn available(game: &Game) -> bool {
    binary_path(game).is_some()
}

pub fn launch(game: &Game) -> Result<()> {
    let path = match binary_path(game) {
        Some(path) => path,
        None => bail!(
            "{} isn't built: run `cargo build -p {}`",
            game.name,
            game.bin
        ),
    };

    let status = Command::new(path).status()?;
    if !status.success() {
        bail!("{} exited with {}", game.name, status);
    }

    Ok(())
}

fn binary_path(game: &Game) -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("TERMCADE_GAMES_DIR") {
        let path = PathBuf::from(dir).join(game.bin);
        if path.is_file() {
            return Some(path);
        }
    }

    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    let path = dir.join(game.bin);

    path.is_file().then_some(path)
}
