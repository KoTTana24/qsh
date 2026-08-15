use mlua::Lua;

use super::{Config, Theme};

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

    Ok(Config {
        theme: Theme { greeting },
    })
}
