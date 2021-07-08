use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

static FALLBACK_DIR: &str = "~/entry_data";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    data_directory: String,
}

impl Config {
    pub fn data_directory(&self) -> String {
        return shellexpand::tilde(&self.data_directory).to_string();
    }

    pub fn set_data_directory(&mut self, value: String) {
        self.data_directory = value;
    }
}

impl Default for Config {
    /// Tries to default to the config directory for all data, but falls back to `~/entry_data`.
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("rs", "entry", "entry");
        let data_directory = project_dirs.map_or(String::from(FALLBACK_DIR), |pd| {
            let config_dir = pd.config_dir().to_str();
            config_dir
                .map(String::from)
                .unwrap_or_else(|| String::from(FALLBACK_DIR))
        });
        Self { data_directory }
    }
}
