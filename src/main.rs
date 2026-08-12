mod builtin;
mod config;
mod expand;
mod history;
mod parser;
mod theme;

use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

use history::History;
use whoami;

fn get_username() -> String {
    whoami::username()
}

fn format_user_path(full_path: &Path) -> String {
    if let Some(home) = env::home_dir() {
        if let Ok(relative) = full_path.strip_prefix(&home) {
            if relative.as_os_str().is_empty() {
                return "~".to_string();
            }

            return format!("~/{}", relative.display());
        }
    }

    full_path.display().to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config();
    let mut history = History::new();

    loop {
        let username = get_username();
        let path = format_user_path(&env::current_dir()?);

        let greeting = theme::format_greeting(&config.theme.greeting, &username, &path);

        print!("{}", greeting);
        io::stdout().flush()?;

        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        history.add(input.clone());

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Parse command
        let parsed = match parser::parse(input) {
            Some(command) => command,
            None => continue,
        };

        let command = parsed.command;
        let args = parsed.args;

        // Convert Vec<String> -> Vec<&str>
        let args_ref: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();

        // Try builtin command first
        if let Some(result) = builtin::execute_builtin(&command, &args_ref) {
            if let Err(error) = result {
                eprintln!("qsh: {}: {}", command, error);
            }

            continue;
        }

        // Run external command
        match Command::new(&command).args(&args_ref).status() {
            Ok(status) => {
                if !status.success() {
                    eprintln!("qsh: process exited with {}", status);
                }
            }

            Err(error) => {
                eprintln!("qsh: {}: {}", command, error);
            }
        }
    }
}
