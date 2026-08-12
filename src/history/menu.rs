use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};

use std::io::{self, Write};

use super::fuzzy;

pub fn open(history: &[String]) -> Option<String> {
    if history.is_empty() {
        return None;
    }

    enable_raw_mode().unwrap();

    let mut query = String::new();
    let mut selected = 0;

    loop {
        let filtered: Vec<&String> = history
            .iter()
            .rev()
            .filter(|cmd| fuzzy::matches(&query, cmd))
            .collect();

        if selected >= filtered.len() && !filtered.is_empty() {
            selected = filtered.len() - 1;
        }

        draw(&filtered, selected, &query);

        match event::read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    query.push(c);
                    selected = 0;
                }

                KeyCode::Backspace => {
                    query.pop();
                    selected = 0;
                }

                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }

                KeyCode::Down => {
                    if selected + 1 < filtered.len() {
                        selected += 1;
                    }
                }

                KeyCode::Enter => {
                    disable_raw_mode().unwrap();

                    println!();

                    return filtered.get(selected).map(|x| (*x).clone());
                }

                KeyCode::Esc => {
                    disable_raw_mode().unwrap();

                    println!();

                    return None;
                }

                _ => {}
            },

            _ => {}
        }
    }
}

fn draw(commands: &[&String], selected: usize, query: &str) {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine)
    )
    .unwrap();

    print!("qsh: command history");

    if !query.is_empty() {
        print!(" > {}", query);
    }

    println!();

    for (i, command) in commands.iter().take(10).enumerate() {
        if i == selected {
            println!("> {}", command);
        } else {
            println!("  {}", command);
        }
    }

    stdout.flush().unwrap();
}
