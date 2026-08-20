use std::sync::{Arc, Mutex};

use mlua::Lua;

use super::{api, context::PluginContext, loader, types::PluginInfo};

pub struct PluginManager {
    pub lua: Lua,

    pub context: Arc<Mutex<PluginContext>>,

    pub plugins: Vec<PluginInfo>,
}

impl PluginManager {
    pub fn new() -> Self {
        let lua = Lua::new();

        let context = Arc::new(Mutex::new(PluginContext::default()));

        if let Err(error) = api::register(&lua, context.clone()) {
            eprintln!("qsh: failed to initialize plugin api: {}", error);
        }

        Self {
            lua,

            context,

            plugins: Vec::new(),
        }
    }

    pub fn load_plugins(&mut self, plugins: &[String]) {
        for plugin in plugins {
            match loader::load(plugin, &self.lua, self.context.clone()) {
                Some(info) => {
                    self.plugins.push(info);
                }

                None => {
                    eprintln!("qsh: failed to load plugin '{}'", plugin);
                }
            }
        }
    }

    pub fn execute_command(&self, name: &str, args: Vec<String>) -> Option<bool> {
        let context = self.context.lock().unwrap();

        let command = context.find_command(name)?;

        Some(command.execute(&self.lua, args))
    }

    pub fn list_plugins(&self) -> &[PluginInfo] {
        &self.plugins
    }
}
