use std::process::{Command, Stdio};

use crate::parser::Command as ShellCommand;

pub fn execute(commands: Vec<ShellCommand>) {
    let mut children = Vec::new();

    let mut previous_stdout = None;

    for (index, command) in commands.iter().enumerate() {
        let mut process = Command::new(&command.program);

        process.args(&command.args);

        if let Some(stdout) = previous_stdout.take() {
            process.stdin(Stdio::from(stdout));
        }

        if index < commands.len() - 1 {
            process.stdout(Stdio::piped());
        }

        match process.spawn() {
            Ok(mut child) => {
                previous_stdout = child.stdout.take();

                children.push(child);
            }

            Err(error) => {
                eprintln!("qsh: {}: {}", command.program, error);

                return;
            }
        }
    }

    for mut child in children {
        let _ = child.wait();
    }
}
