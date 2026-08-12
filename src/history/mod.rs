pub mod fuzzy;
pub mod menu;

use std::fs;
use std::path::PathBuf;

pub struct History {
    pub entries: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Self::load(),
        }
    }

    fn path() -> PathBuf {
        dirs::home_dir().unwrap().join(".qsh_history")
    }

    fn load() -> Vec<String> {
        match fs::read_to_string(Self::path()) {
            Ok(content) => content.lines().map(|x| x.to_string()).collect(),

            Err(_) => Vec::new(),
        }
    }

    pub fn add(&mut self, command: String) {
        let command = command.trim();

        if command.is_empty() {
            return;
        }

        self.entries.push(command.to_string());

        let _ = fs::write(Self::path(), self.entries.join("\n"));
    }
}
