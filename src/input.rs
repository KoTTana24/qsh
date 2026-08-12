use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io::{self, Write};

use crate::history::menu::open;

pub fn clear_current_line() {
    print!("\r\x1b[2K");
}

pub fn read_input(history: &[String]) -> Option<String> {
    enable_raw_mode().unwrap();

    let mut input = String::new();

    loop {
        match event::read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(c) => {
                    input.push(c);

                    print!("{}", c);
                    io::stdout().flush().unwrap();
                }

                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        io::stdout().execute(cursor::MoveLeft(1)).unwrap();

                        print!(" ");

                        io::stdout().execute(cursor::MoveLeft(1)).unwrap();

                        io::stdout().flush().unwrap();
                    }
                }
                KeyCode::Up => {
                    disable_raw_mode().unwrap();

                    // перейти на новую строку перед меню
                    println!();

                    if let Some(command) = open(history) {
                        input = command;

                        clear_current_line();

                        print!("{}", input);

                        io::stdout().flush().unwrap();
                    }

                    enable_raw_mode().unwrap();
                }

                KeyCode::Enter => {
                    println!();

                    disable_raw_mode().unwrap();

                    return Some(input);
                }

                KeyCode::Esc => {
                    disable_raw_mode().unwrap();

                    return None;
                }

                _ => {}
            },

            _ => {}
        }
    }
}
