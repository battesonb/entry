use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub create_file: bool,
    pub default_note_name: String,
    pub minute_bucket_size: u32,
    pub note_directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            create_file: false,
            default_note_name: String::from("default"),
            minute_bucket_size: 15,
            note_directory: String::from("~/entries")
        }
    }
}

#[derive(StructOpt)]
pub enum ConfigCommand {
    #[structopt(about = "Get the value for the given config key")]
    Get {
        key: String
    },
    #[structopt(about = "Set the value for the given config key")]
    Set {
        key: String,
        value: String
    },
    #[structopt(about = "List all config keys")]
    List {}
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Create a new entry")]
    New {
        #[structopt(short = "t", long = "time", default_value = "now")]
        time: String,
        entry_name: Option<String>,
    },
    #[structopt(about = "Find entries with the given text")]
    Find {
        text: Option<String>,
        entry_name: Option<String>,
    },
    #[structopt(about = "Configure your default entry rules")]
    Config {
        #[structopt(subcommand)]
        cmd: ConfigCommand
    }
}

#[derive(StructOpt)]
#[structopt(about = "a quick note-taking tool")]
pub struct Entry {
    #[structopt(subcommand)]
    pub cmd: Command
}
