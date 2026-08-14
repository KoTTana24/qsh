use crate::parser::Command as ShellCommand;
use std::process::Command;

use super::redirect;

use crate::builtin;

pub fn execute(command: ShellCommand) -> bool {
    if let Some(result) = builtin::execute(&command.program, &command.args) {
        return result;
    }

    let mut process = Command::new(&command.program);

    process.args(&command.args);

    redirect::apply_stdin(&mut process, command.stdin);

    redirect::apply_stdout(&mut process, command.stdout);

    let result = process.status();

    match result {
        Ok(status) => {
            if !status.success() {
                eprintln!("qsh: exited with {}", status);
            }

            status.success()
        }

        Err(error) => {
            eprintln!("qsh: {}: {}", command.program, error);

            false
        }
    }
}
