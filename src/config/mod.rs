pub mod lua;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub greeting: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme {
                greeting: "{current_directory}@{username} >".to_string(),
            },
        }
    }
}

pub fn load_config() -> Config {
    let path = config_path();

    let code =
        std::fs::read_to_string(&path).unwrap_or_else(|_| include_str!("default.lua").to_string());

    match lua::parse(&code) {
        Ok(config) => config,

        Err(error) => {
            eprintln!("qsh config error: {}", error);

            Config::default()
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("qsh")
        .join("config.lua")
}
