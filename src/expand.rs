use std::env;
use std::path::PathBuf;

pub fn expand_path(path: &str) -> String {
    if path == "~" {
        return home_dir();
    }

    if let Some(rest) = path.strip_prefix("~/") {
        return format!("{}/{}", home_dir(), rest);
    }

    path.to_string()
}

fn home_dir() -> String {
    env::home_dir()
        .unwrap_or(PathBuf::from("/"))
        .display()
        .to_string()
}
