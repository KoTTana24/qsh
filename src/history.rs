use std::io::Write;

pub struct History {
    commands: Vec<String>,
}
impl History {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    pub fn add(&mut self, command: String) {
        self.commands.push(command);
    }
    pub fn get(&self, index: usize) -> Option<&str> {
        self.commands.get(index).map(|c| c.as_str())
    }
    pub fn len(&self) -> usize {
        self.commands.len()
    }
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

pub fn load_history() -> History {
    let mut history = History::new();
    if let Ok(contents) = std::fs::read_to_string("~/.qsh_history") {
        for line in contents.lines() {
            history.add(line.to_string());
        }
    }
    history
}

pub fn save_history(history: &History) {
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("~/.qsh_history")
    {
        for command in &history.commands {
            writeln!(file, "{}", command).ok();
        }
    }
}
