use clap::Parser;
use expanduser::expanduser;

use crate::config::Config;

#[derive(Debug, Parser)]
struct TypieCli {
    #[arg(
        short,
        long,
        value_name = "file",
        default_value = "~/.config/typie/config.json"
    )]
    config: Option<String>,

    #[arg(short, long, value_name = "boolean")]
    show_keyboard: Option<bool>,
}

pub fn run() -> Result<Config, String> {
    let typie_cli = TypieCli::parse();

    let config_path = expanduser(&typie_cli.config.unwrap())
        .map_err(|err| format!("Failed to parse config path: {err}"))?;

    let mut config = Config::load(&config_path)?;

    if let Some(show_keyboard) = typie_cli.show_keyboard.as_ref() {
        config.show_keyboard = show_keyboard.to_owned();
    }

    Ok(config)
}
