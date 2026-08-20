use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use mlua::Lua;

use crate::version::QSH_VERSION;

use super::{
    api,
    context::PluginContext,
    version::{Version, compatible},
};

pub fn load(name: &str, lua: &Lua, context: Arc<Mutex<PluginContext>>) {
    let path = plugin_path(name);

    let code = match fs::read_to_string(&path) {
        Ok(code) => code,

        Err(error) => {
            eprintln!("qsh: failed to load plugin '{}': {}", name, error);

            return;
        }
    };

    if let Err(error) = lua.load(&code).exec() {
        eprintln!("qsh: plugin '{}' error: {}", name, error);

        return;
    }

    let globals = lua.globals();

    let plugin = match globals.get::<mlua::Table>("plugin") {
        Ok(plugin) => plugin,

        Err(_) => {
            eprintln!("qsh: plugin '{}' has no metadata", name);

            return;
        }
    };

    let plugin_name = plugin.get::<String>("name").unwrap_or(name.to_string());

    let plugin_version = plugin
        .get::<String>("version")
        .unwrap_or("0.0.0".to_string());

    //
    // Check qsh version
    //

    if let Ok(qsh_table) = plugin.get::<mlua::Table>("qsh") {
        if let Ok(required) = qsh_table.get::<String>("min_version") {
            let current = Version::parse(QSH_VERSION);

            let needed = Version::parse(&required);

            if !compatible(&current, &needed) {
                eprintln!("qsh: plugin '{}' requires qsh >= {}", plugin_name, required);

                return;
            }
        }
    }

    //
    // Register API
    //

    if let Err(error) = api::register(lua, context) {
        eprintln!("qsh: plugin '{}' API error: {}", plugin_name, error);

        return;
    }

    println!("Loaded {} v{}", plugin_name, plugin_version);
}

fn plugin_path(name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("qsh")
        .join("plugins")
        .join(name)
        .join("plugin.lua")
}
