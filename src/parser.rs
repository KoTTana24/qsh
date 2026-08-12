use crate::expand;

pub struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl ParsedCommand {
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self { command, args }
    }
}

pub fn parse(input: &str) -> Option<ParsedCommand> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    let command = parts[0].to_string();

    let args = parts[1..]
        .iter()
        .map(|arg| expand::expand_path(arg))
        .collect();

    Some(ParsedCommand::new(command, args))
}
