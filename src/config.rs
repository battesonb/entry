use std::str::FromStr;

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::errors::EntryError;

static FALLBACK_DIR: &'static str = "~/entry_data";

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub data_directory: String,
}

impl Default for Config {
    /// Tries to default to the config directory for all data, but falls back to `~/entry_data`.
    fn default() -> Self {
        let project_dirs = ProjectDirs::from("rs", "entry", "entry");
        let data_directory = project_dirs.map_or(String::from(FALLBACK_DIR), |pd| {
            let config_dir = pd.config_dir().to_str();
            return config_dir
                .map(|cd| String::from(cd))
                .unwrap_or(String::from(FALLBACK_DIR));
        });
        Self { data_directory }
    }
}
