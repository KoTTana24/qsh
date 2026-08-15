use std::env;
use std::path::PathBuf;

use glob::glob;

fn expand_home(word: &str) -> String {
    if word == "~" {
        if let Some(home) = std::env::home_dir() {
            return home.display().to_string();
        }
    }

    if let Some(rest) = word.strip_prefix("~/") {
        if let Some(home) = std::env::home_dir() {
            return format!("{}/{}", home.display(), rest);
        }
    }

    word.to_string()
}

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

pub fn expand_word(word: &str) -> Vec<String> {
    let word = expand_home(&expand_variables(word));
    if contains_glob(&word) {
        let mut result = Vec::new();

        if let Ok(paths) = glob(&word) {
            for path in paths.flatten() {
                result.push(path.display().to_string());
            }
        }

        if !result.is_empty() {
            return result;
        }
    }

    vec![word]
}

fn expand_variables(word: &str) -> String {
    let mut result = String::new();

    let chars: Vec<char> = word.chars().collect();

    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            let mut name = String::new();

            i += 1;

            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                name.push(chars[i]);

                i += 1;
            }

            if let Ok(value) = env::var(&name) {
                result.push_str(&value);
            }

            continue;
        }

        result.push(chars[i]);

        i += 1;
    }

    result
}

fn contains_glob(word: &str) -> bool {
    word.contains('*') || word.contains('?') || word.contains('[')
}
