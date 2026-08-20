use std::collections::HashMap;

use super::{Config, Theme};

use mlua::Lua;

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

    let theme = Theme { greeting };

    let mut aliases = HashMap::new();

    if let Ok(table) = globals.get::<mlua::Table>("aliases") {
        for pair in table.pairs::<String, String>() {
            if let Ok((name, command)) = pair {
                aliases.insert(name, command);
            }
        }
    }

    let mut plugins = Vec::new();

    if let Ok(table) = globals.get::<mlua::Table>("plugins") {
        if let Ok(enabled) = table.get::<mlua::Table>("enabled") {
            for plugin in enabled.sequence_values::<String>() {
                if let Ok(plugin) = plugin {
                    plugins.push(plugin);
                }
            }
        }
    }

    Ok(Config {
        theme,

        aliases,

        plugins,
    })
}
