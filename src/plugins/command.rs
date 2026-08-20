use mlua::{Lua, RegistryKey};

pub struct PluginCommand {
    pub name: String,

    pub callback: RegistryKey,
}

impl PluginCommand {
    pub fn execute(&self, lua: &Lua, args: Vec<String>) -> bool {
        match lua.registry_value::<mlua::Function>(&self.callback) {
            Ok(function) => match function.call::<()>(args) {
                Ok(_) => true,

                Err(error) => {
                    eprintln!("plugin command error: {}", error);

                    false
                }
            },

            Err(error) => {
                eprintln!("plugin command error: {}", error);

                false
            }
        }
    }
}
