use std::collections::HashMap;

use super::events::Events;

pub struct PluginContext {
    pub aliases: HashMap<String, String>,
    pub events: Events,
}

impl Default for PluginContext {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
            events: Events::new(),
        }
    }
}
