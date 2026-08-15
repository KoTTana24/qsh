use super::{Config, Theme};
use mlua::Lua;
use std::collections::HashMap;

pub fn parse(code: &str) -> Result<Config, mlua::Error> {
    let lua = Lua::new();

    lua.load(code).exec()?;

    let globals = lua.globals();

    let greeting = match globals.get::<mlua::Table>("theme") {
        Ok(theme) => theme
            .get::<String>("greeting")
            .unwrap_or_else(|_| Config::default().theme.greeting),

        Err(_) => Config::default().theme.greeting,
    };

    let mut aliases = HashMap::new();

    if let Ok(table) = globals.get::<mlua::Table>("aliases") {
        for pair in table.pairs::<String, String>() {
            if let Ok((name, command)) = pair {
                aliases.insert(name, command);
            }
        }
    }
    Ok(Config {
        theme: Theme { greeting },

        aliases,
    })
}
