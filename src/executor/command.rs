use std::process::Command;

use crate::parser::{Command as ShellCommand, Redirect};

use super::redirect;

pub fn execute(command: ShellCommand) {
    let mut process = Command::new(&command.program);

    process.args(&command.args);

    redirect::apply_stdout(&mut process, command.stdout);

    match process.status() {
        Ok(status) => {
            if !status.success() {
                eprintln!("qsh: exited with {}", status);
            }
        }

        Err(error) => {
            eprintln!("qsh: {}: {}", command.program, error);
        }
    }
}
