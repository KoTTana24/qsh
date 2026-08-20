use crate::parser::Command as ShellCommand;
use std::process::Command;

use super::redirect;

use crate::builtin;

use crate::plugins::manager::PluginManager;
pub fn execute(command: ShellCommand, plugins: &PluginManager) -> bool {
    let allowed =
        plugins
            .context
            .lock()
            .unwrap()
            .events
            .run_before_command(
                &plugins.lua,
                command.program.clone(),
            );


    if !allowed {
        return false;
    }
    if let Some(result) = plugins.execute_command(&command.program, command.args.clone()) {
        return result;
    }

    if let Some(result) = builtin::execute(&command.program, &command.args) {
        return result;
    }

    let mut process = Command::new(&command.program);

    process.args(&command.args);

    redirect::apply_stdin(&mut process, command.stdin);

    redirect::apply_stdout(&mut process, command.stdout);

    match process.status() {
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
