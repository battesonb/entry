mod args;
mod config;
mod errors;
mod schema;

use args::{Command, ConfigCommand, Entry, SchemaCommand};
use config::Config;
use schema::{Schema, SchemaCount, SchemaDataType, SchemaType};

use exitfailure::ExitFailure;
use failure::{err_msg, ResultExt};
use std::{fs, io, str::FromStr};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let mut config: Config = confy::load("entry")?;
    initialize(&config)?;

    let args = Entry::from_args();
    match args.cmd {
        Command::Schema { cmd: subcommand } => match subcommand {
            SchemaCommand::New => {
                let mut schema = Schema::default();
                println!("Enter a name for the schema");
                let schema_name = read_line();
                loop {
                    println!("Enter a name for the new field (or nothing to finish up):");
                    let field_name = read_line();
                    if field_name.is_empty() {
                        break;
                    }
                    println!("found {}", field_name);
                    println!("Enter a type for the new field (string, number, date):");
                    loop {
                        if let Ok(data_type) = SchemaDataType::from_str(&read_line()) {
                            println!("Is this an array? (y/n)");
                            let ans = read_line().to_lowercase();
                            schema.insert(
                                field_name,
                                SchemaType {
                                    count: if ans == "y" {
                                        SchemaCount::Many
                                    } else {
                                        SchemaCount::One
                                    },
                                    data_type,
                                },
                            );
                            break;
                        } else {
                            println!("Invalid data type received, try again.");
                        }
                    }
                }
                if !schema.is_empty() {
                    println!("Saving schema...");
                    schema
                        .save(&format!("{}/schema", &config.data_directory), &schema_name)
                        .unwrap();
                } else {
                    println!("Schema is empty, not saving.");
                }
            }
            SchemaCommand::List => {
                Schema::list(&config).iter().for_each(|schema_name| {
                    println!("{}", schema_name);
                });
            }
            SchemaCommand::Show { schema_name } => {
                if let Ok(schema) =
                    Schema::load(&format!("{}/schema", &config.data_directory), &schema_name)
                {
                    schema.print();
                }
            }
            _ => todo!("schema command"),
        },
        Command::Config { cmd } => match cmd {
            ConfigCommand::Get { key } => match key.as_str() {
                "data_directory" => println!("{}", config.data_directory),
                _ => return Err(err_msg("invalid key, failed to retrieve config"))?,
            },
            ConfigCommand::Set { key, value } => {
                match key.as_str() {
                    "data_directory" => {
                        let res: Result<String, _> = value.parse();
                        match res {
                            Ok(v) => config.data_directory = v,
                            _ => return Err(err_msg("invalid value, failed to set config"))?,
                        }
                    }
                    _ => return Err(err_msg("invalid key, failed to retrieve config"))?,
                }
                confy::store("entry", config).with_context(|_| "failed to save config")?;
            }
            ConfigCommand::List {} => {
                println!("data_directory={}", config.data_directory);
            }
        },
        _ => todo!("command"),
    }
    Ok(())
}

fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    return s.trim().to_string();
}

fn initialize(config: &Config) -> Result<(), io::Error> {
    let path = format!("{}/schema", config.data_directory);
    return fs::create_dir_all(path);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn invalid_minute() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 12:60");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("invalid minute input"));
        Ok(())
    }
}
