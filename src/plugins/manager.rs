use std::sync::{Arc, Mutex};

use mlua::Lua;

use super::{context::PluginContext, loader};

pub struct PluginManager {
    pub lua: Lua,

    pub context: Arc<Mutex<PluginContext>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),

            context: Arc::new(Mutex::new(PluginContext::default())),
        }
    }

    pub fn load_plugins(&self, plugins: &[String]) {
        for plugin in plugins {
            loader::load(plugin, &self.lua, self.context.clone());
        }
    }
    pub fn execute_command(&self, name: &str, args: Vec<String>) -> Option<bool> {
        let context = self.context.lock().unwrap();

        let command = context.find_command(name)?;

        Some(command.execute(&self.lua, args))
    }
}
