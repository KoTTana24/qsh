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

pub fn expand_word(word: &str) -> String {
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
            } else {
                // unknown varible -> none
            }

            continue;
        }

        result.push(chars[i]);

        i += 1;
    }

    result
}
