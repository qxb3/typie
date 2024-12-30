use std::{fs, path::PathBuf};
use serde::Deserialize;

fn show_keyboard() -> bool { true }

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "show_keyboard")]
    pub show_keyboard: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_keyboard: show_keyboard()
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, String> {
        match fs::read_to_string(path) {
            Ok(config) => {
                let config: Config = serde_json::from_str(&config)
                    .map_err(|err| format!("Error loading the config: {err}"))?;

                Ok(config)
            },
            Err(_) => Ok(Config::default())
        }
    }
}
