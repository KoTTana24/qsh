use mlua::RegistryKey;

pub struct Events {
    pub on_start: Vec<RegistryKey>,
    pub before_command: Vec<RegistryKey>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            on_start: Vec::new(),
            before_command: Vec::new(),
        }
    }
}
