use structopt::StructOpt;

#[derive(StructOpt)]
pub enum ConfigCommand {
    #[structopt(about = "Get the value for the given config key")]
    Get { key: String },
    #[structopt(about = "Set the value for the given config key")]
    Set { key: String, value: String },
    #[structopt(about = "List all config keys")]
    List,
}

#[derive(StructOpt)]
pub enum SchemaCommand {
    #[structopt(about = "Generate a new schema interactively")]
    New,
    #[structopt(about = "Print the given schema")]
    Show { schema_name: String },
    #[structopt(about = "List all schemas")]
    List,
    #[structopt(about = "Remove a given schema")]
    Remove { schema_name: String },
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(about = "Generate, list or show schemas")]
    Schema {
        #[structopt(subcommand)]
        cmd: SchemaCommand,
    },
    #[structopt(about = "Generate a new entry for the given schema")]
    For { schema_name: String },
    #[structopt(about = "Print the last entry")]
    Last,
    #[structopt(about = "Configure your default entry rules")]
    Config {
        #[structopt(subcommand)]
        cmd: ConfigCommand,
    },
}

#[derive(StructOpt)]
#[structopt(about = "A tool for generating JSON-formatted data from a local schema")]
pub struct Entry {
    #[structopt(subcommand)]
    pub cmd: Command,
}
