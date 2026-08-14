pub mod command;
pub mod pipeline;
pub mod redirect;

use crate::parser::Statement;

pub fn execute(statement: Statement) -> bool {
    match statement {
        Statement::Command(command) => command::execute(command),

        Statement::Pipeline(pipeline) => pipeline::execute(pipeline.commands),

        Statement::Sequence(commands) => {
            let mut success = true;

            for command in commands {
                success = execute(command);
            }

            success
        }

        Statement::And(left, right) => {
            if execute(*left) {
                execute(*right)
            } else {
                false
            }
        }

        Statement::Or(left, right) => {
            if execute(*left) {
                true
            } else {
                execute(*right)
            }
        }
    }
}
