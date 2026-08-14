use std::{
    fs::{File, OpenOptions},
    process::{Command, Stdio},
};

use crate::parser::Redirect;

pub fn apply_stdout(command: &mut Command, redirect: Option<Redirect>) {
    let Some(redirect) = redirect else {
        return;
    };

    match redirect {
        Redirect::Write(path) => {
            let file = File::create(path).expect("failed to create file");

            command.stdout(Stdio::from(file));
        }

        Redirect::Append(path) => {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("failed to open file");

            command.stdout(Stdio::from(file));
        }
    }
}
