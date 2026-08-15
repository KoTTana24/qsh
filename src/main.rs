mod alias;
mod builtin;
mod config;
mod executor;
mod expand;
mod history;
mod input;
mod parser;
mod theme;

use std::env;
use std::io::{self, Write};
use std::path::Path;

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
        let raw_input = match input::read_input(&greeting, &history.entries) {
            Some(input) => input,
            None => continue,
        };

        let raw_input = raw_input.trim();

        if raw_input.is_empty() {
            continue;
        }
        if raw_input == "exit" {
            break;
        }

        let input = alias::expand(raw_input, &config.aliases);

        let tokens = parser::tokenize(&input);

        history.add(input.to_string());

        // new parser

        let tokens = parser::tokenize(&input);

        let ast = parser::parse(&tokens);

        if let Some(ref ast) = ast {
            executor::execute(ast.clone());
        }
    }

    Ok(())
}
