use std::collections::HashMap;

pub fn expand(input: &str, aliases: &HashMap<String, String>) -> String {
    let mut parts = input.split_whitespace();

    let Some(command) = parts.next() else {
        return input.to_string();
    };

    if let Some(alias) = aliases.get(command) {
        let rest = parts.collect::<Vec<_>>().join(" ");

        if rest.is_empty() {
            alias.clone()
        } else {
            format!("{} {}", alias, rest)
        }
    } else {
        input.to_string()
    }
}
