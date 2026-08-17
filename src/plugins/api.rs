use std::sync::{Arc, Mutex};

use mlua::{Lua, Result};

use super::context::PluginContext;

pub fn register(lua: &Lua, context: Arc<Mutex<PluginContext>>) -> Result<()> {
    let globals = lua.globals();

    let qsh = lua.create_table()?;

    let print = lua.create_function(|_, message: String| {
        println!("{}", message);

        Ok(())
    })?;

    qsh.set("print", print)?;

    let alias_context = context.clone();

    let register_alias = lua.create_function(move |_, (name, command): (String, String)| {
        let mut ctx = alias_context.lock().unwrap();

        ctx.aliases.insert(name, command);

        Ok(())
    })?;
    let events = context.clone();

    let on_start = lua.create_function(move |lua, callback: mlua::Function| {
        let key = lua.create_registry_value(callback)?;

        events.lock().unwrap().events.on_start.push(key);

        Ok(())
    })?;

    globals.set("on_start", on_start)?;

    qsh.set("register_alias", register_alias)?;

    globals.set("qsh", qsh)?;

    Ok(())
}
