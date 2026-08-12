pub fn format_greeting(template: &str, username: &str, directory: &str) -> String {
    template
        .replace("{username}", username)
        .replace("{current_directory}", directory)
}
