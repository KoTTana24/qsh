pub mod command;
pub mod pipeline;
pub mod redirect;

use crate::parser::Statement;

pub fn execute(statement: Statement) {
    match statement {
        Statement::Command(command) => {
            command::execute(command);
        }

        Statement::Pipeline(pipeline) => {
            pipeline::execute(pipeline.commands);
        }

        Statement::Sequence(commands) => {
            for command in commands {
                execute(command);
            }
        }
    }
}
