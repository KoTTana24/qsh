use super::{command::PluginCommand, events::Events};
use std::collections::HashMap;

pub struct PluginContext {
    pub aliases: HashMap<String, String>,

    pub commands: Vec<PluginCommand>,

    pub events: Events,
}

impl PluginContext {
    pub fn find_command(&self, name: &str) -> Option<&PluginCommand> {
        self.commands.iter().find(|cmd| cmd.name == name)
    }
}

impl Default for PluginContext {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),

            commands: Vec::new(),

            events: Events::new(),
        }
    }
}
