use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use mlua::Lua;

use crate::version::QSH_VERSION;

use super::{
    context::PluginContext,
    types::PluginInfo,
    version::{Version, compatible},
};

pub fn load(name: &str, lua: &Lua, _context: Arc<Mutex<PluginContext>>) -> Option<PluginInfo> {
    let path = plugin_path(name);

    let code = match fs::read_to_string(&path) {
        Ok(code) => code,

        Err(error) => {
            eprintln!("qsh: failed to load plugin '{}': {}", name, error);

            return None;
        }
    };

    if let Err(error) = lua.load(&code).exec() {
        eprintln!("qsh: plugin '{}' error: {}", name, error);

        return None;
    }

    let globals = lua.globals();

    let plugin = match globals.get::<mlua::Table>("plugin") {
        Ok(plugin) => plugin,

        Err(_) => {
            eprintln!("qsh: plugin '{}' has no metadata", name);

            return None;
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

                return None;
            }
        }
    }

    let info = PluginInfo {
        name: plugin_name.clone(),

        version: plugin_version.clone(),

        author: plugin
            .get::<String>("author")
            .unwrap_or("unknown".to_string()),

        description: plugin
            .get::<String>("description")
            .unwrap_or("".to_string()),

        qsh_version: plugin
            .get::<mlua::Table>("qsh")
            .ok()
            .and_then(|table| table.get::<String>("min_version").ok()),

        path: path.display().to_string(),
    };

    println!("Loaded {} v{}", info.name, info.version);

    Some(info)
}

fn plugin_path(name: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("qsh")
        .join("plugins")
        .join(name)
        .join("plugin.lua")
}
