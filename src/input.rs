use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io::{self, Write};

pub fn read_input(prompt: &str, history: &[String]) -> Option<String> {
    enable_raw_mode().unwrap();

    let mut input = String::new();

    let mut history_index = history.len();

    let mut browsing_history = false;

    loop {
        match event::read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    input.push(c);

                    browsing_history = false;

                    print!("{}", c);

                    io::stdout().flush().unwrap();
                }

                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        clear_input(prompt, &input);
                    }
                }

                KeyCode::Up => {
                    if history.is_empty() {
                        continue;
                    }

                    browsing_history = true;

                    if history_index > 0 {
                        history_index -= 1;
                    }

                    replace_input(prompt, &mut input, &history[history_index]);
                }

                KeyCode::Down => {
                    if !browsing_history {
                        continue;
                    }

                    if history_index + 1 < history.len() {
                        history_index += 1;

                        replace_input(prompt, &mut input, &history[history_index]);
                    } else {
                        history_index = history.len();

                        replace_input(prompt, &mut input, "");
                    }
                }

                KeyCode::Esc => {
                    browsing_history = false;

                    history_index = history.len();

                    replace_input(prompt, &mut input, "");
                }

                KeyCode::Enter => {
                    disable_raw_mode().unwrap();

                    println!();

                    return Some(input);
                }

                _ => {}
            },

            _ => {}
        }
    }
}

fn replace_input(prompt: &str, input: &mut String, value: &str) {
    *input = value.to_string();

    clear_input(prompt, input);
}

fn clear_input(prompt: &str, input: &str) {
    let mut stdout = io::stdout();

    stdout.execute(cursor::MoveToColumn(0)).unwrap();

    // очищаем только текущую строку
    print!("\x1b[2K");

    // возвращаем prompt и ввод
    print!("{}{}", prompt, input);

    stdout.flush().unwrap();
}
