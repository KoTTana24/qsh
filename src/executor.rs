use std::process::Command;

use crate::parser::{Command as ShellCommand, Statement};

pub fn execute(statement: Statement) {
    match statement {
        Statement::Command(command) => {
            execute_command(command);
        }

        Statement::Pipeline(pipeline) => {
            println!("pipeline execution not implemented yet: {:?}", pipeline);
        }

        Statement::Sequence(commands) => {
            for command in commands {
                execute(command);
            }
        }
    }
}

fn execute_command(command: ShellCommand) {
    let result = Command::new(&command.program).args(&command.args).status();

    match result {
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
