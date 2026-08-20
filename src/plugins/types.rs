#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,

    pub version: String,

    pub author: String,

    pub description: String,

    pub qsh_version: Option<String>,

    pub path: String,
}
