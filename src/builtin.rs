pub fn execute_builtin(command: &str, args: &[&str]) -> Option<Result<(), String>> {
    match command {
        "echo" => Some(echo(args)),
        "cd" => Some(cd(args)),
        _ => None,
    }
}

use std::env;

pub fn echo(args: &[&str]) -> Result<(), String> {
    println!("{}", args.join(" "));
    Ok(())
}

pub fn cd(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        let home = env::var("HOME").unwrap_or_default();
        env::set_current_dir(&home).map_err(|e| e.to_string())
    } else {
        env::set_current_dir(args[0]).map_err(|e| e.to_string())
    }
}
