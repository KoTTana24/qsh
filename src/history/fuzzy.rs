pub fn matches(query: &str, text: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let query = query.to_lowercase();
    let text = text.to_lowercase();

    let mut chars = text.chars();

    for q in query.chars() {
        if chars.find(|c| *c == q).is_none() {
            return false;
        }
    }

    true
}
