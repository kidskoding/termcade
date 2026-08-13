use std::fs;

use serde::Deserialize;

const APP_DIR: &str = "tetrotui";
const CONFIG_FILE: &str = "config.toml";

#[derive(Deserialize, Default)]
pub struct Config {
    pub das_ms: u64,
    pub arr_ms: u64,
}

pub fn load() -> Option<Config> {
    let dir = dirs::config_dir()?.join(APP_DIR);
    fs::create_dir_all(&dir).ok()?;

    let path = dir.join(CONFIG_FILE);
    let contents = fs::read_to_string(path).ok()?;

    toml::from_str::<Config>(&contents).ok()
}
