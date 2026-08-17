use std::sync::{Arc, Mutex};

use super::{context::PluginContext, loader};

pub fn load(plugins: &[String]) -> Arc<Mutex<PluginContext>> {
    let context = Arc::new(Mutex::new(PluginContext::default()));

    for plugin in plugins {
        loader::load(plugin, context.clone());
    }

    context
}
