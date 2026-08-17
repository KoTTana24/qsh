use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::plugins::context;

use super::context::PluginContext;

use mlua::Lua;

use super::api;

pub fn load(name: &str, context: Arc<Mutex<PluginContext>>) {
    let path = plugin_path(name);

    let code = match fs::read_to_string(&path) {
        Ok(code) => code,

        Err(error) => {
            eprintln!("Failed loading {}: {}", name, error);

            return;
        }
    };

    let lua = Lua::new();

    if let Err(error) = api::register(&lua, context.clone()) {
        eprintln!("Plugin API error: {}", error);

        return;
    }

    match lua.load(&code).exec() {
        Ok(_) => {
            println!("Loaded plugin: {}", name);
        }

        Err(error) => {
            eprintln!("Plugin {} error: {}", name, error);
        }
    }
    let globals = lua.globals();

    if let Ok(plugin) = globals.get::<mlua::Table>("plugin") {
        let name = plugin.get::<String>("name").unwrap_or("unknown".into());

        let version = plugin.get::<String>("version").unwrap_or("0.0.0".into());

        println!("Loaded {} v{}", name, version);
    }
}

fn plugin_path(name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("qsh")
        .join("plugins")
        .join(name)
        .join("plugin.lua")
}
