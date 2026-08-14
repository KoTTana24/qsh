use std::env;

pub fn execute(command: &str, args: &[String]) -> Option<bool> {
    match command {
        "echo" => {
            echo(args);

            Some(true)
        }

        "cd" => Some(cd(args)),

        "pwd" => {
            pwd();

            Some(true)
        }

        "exit" => Some(true),

        _ => None,
    }
}

fn echo(args: &[String]) {
    println!("{}", args.join(" "));
}

fn cd(args: &[String]) -> bool {
    if args.len() != 1 {
        eprintln!("cd: expected one argument");

        return false;
    }

    match env::set_current_dir(&args[0]) {
        Ok(_) => true,

        Err(error) => {
            eprintln!("cd: {}", error);

            false
        }
    }
}

fn pwd() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }

        Err(error) => {
            eprintln!("pwd: {}", error);
        }
    }
}
