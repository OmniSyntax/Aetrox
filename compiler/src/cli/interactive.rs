// 👈 (For prompts & menus)
use dialoguer::{Input, Select};

pub struct ProjectConfig {
    pub name: String,
    pub template: String,
}

pub fn prompt_user() -> ProjectConfig {
    // ... paste your interactive dialoguer code here ...
    ProjectConfig { name: "app".into(), template: "web".into() }
}