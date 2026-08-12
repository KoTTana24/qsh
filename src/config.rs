use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub theme: Theme,
}

#[derive(Debug, Deserialize)]
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
    let path = match dirs::config_dir() {
        Some(path) => path.join("qsh/config.toml"),
        None => return Config::default(),
    };

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Config::default(),
    };

    toml::from_str(&content).unwrap_or_else(|_| Config::default())
}
