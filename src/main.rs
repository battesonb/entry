mod args;
mod utils;

use args::{Command, Config, ConfigCommand, Entry};
use utils::{get_datetime, get_directory, get_editor, has_text};

use exitfailure::ExitFailure;
use failure::{err_msg, ResultExt};
use std::{fs, io::{BufRead, BufReader}};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let mut config: Config = confy::load("entry")?;
    let args = Entry::from_args();
    match args.cmd {
        Command::New {entry_name, time} => {
            let datetime = get_datetime(&time, config.minute_bucket_size).with_context(
                |_| "could not create a note"  
            )?;
            let actual_entry_name = match entry_name {
                Some(name) => name,
                _ => config.default_note_name
            };
            let directory = get_directory(&config.note_directory, &actual_entry_name);
            fs::create_dir_all(&directory)?;
            let filepath = format!("{}/{}.txt", directory, datetime.format("%Y-%m-%d-%H-%M"));
            if config.create_file {
                let path = std::path::Path::new(&filepath);
                if !path.exists() {
                    fs::File::create(&filepath)?;
                }
            }
            let editor = get_editor().with_context(|_| "please set your `EDITOR` environment variable to point to your favorite text editor")?;
            std::process::Command::new(editor).arg(&filepath).status()?;
        },
        Command::Find {entry_name, text} => {
            let actual_entry_name = match entry_name {
                Some(name) => name,
                _ => config.default_note_name
            };
            let directory = get_directory(&config.note_directory, &actual_entry_name);
            let dir_entries_result = fs::read_dir(&directory);

            match dir_entries_result {
                Ok(dir_entries) => {
                    for dir_entry in dir_entries {
                        let path = dir_entry.unwrap().path();
                        let file = fs::File::open(&path)?;
                        let reader = BufReader::new(file);
                        match text {
                            Some(ref t) => {
                                if has_text(t, reader.lines()) {
                                    println!("{}", path.display());
                                }
                            }
                            None => {
                                println!("{}", path.display());
                            }
                        }
                    }
                },
                Err(_) => {}
            }
        },
        Command::Config {cmd} => {
            match cmd {
                ConfigCommand::Get {key} => {
                    match key.as_str() {
                        "create_file" => println!("{}", config.create_file),
                        "default_note_name" => println!("{}", config.default_note_name),
                        "minute_bucket_size" => println!("{}", config.minute_bucket_size),
                        "note_directory" => println!("{}", config.note_directory),
                        _ => return Err(err_msg("invalid key, failed to retrieve config"))?
                    }
                },
                ConfigCommand::Set {key, value} => {
                    match key.as_str() {
                        "create_file" => {
                            let res: Result<bool, _> = value.parse();
                            match res {
                                Ok(v) => config.create_file = v,
                                _ => return Err(err_msg("invalid value, failed to set config"))?
                            }
                        },
                        "default_note_name" => {
                            let res: Result<String, _> = value.parse();
                            match res {
                                Ok(v) => config.default_note_name = v,
                                _ => return Err(err_msg("invalid value, failed to set config"))?
                            }
                        },
                        "minute_bucket_size" => {
                            let res: u32 = value.parse::<u32>().with_context(|_| "minute bucket size must be between 0 and 60")?;
                            if res > 60 {
                                return Err(err_msg("minute bucket size must be between 0 and 60"))?;
                            } else {
                                config.minute_bucket_size = res;
                            }
                        },
                        "note_directory" => {
                            let res: Result<String, _> = value.parse();
                            match res {
                                Ok(v) => config.note_directory = v,
                                _ => return Err(err_msg("invalid value, failed to set config"))?
                            }
                        },
                        _ => return Err(err_msg("invalid key, failed to retrieve config"))?
                    }
                    confy::store("entry", config).with_context(|_| "failed to save config")?;
                },
                ConfigCommand::List {} => {
                    println!("create_file={}", config.create_file);
                    println!("default_note_name={}", config.default_note_name);
                    println!("minute_bucket_size={}", config.minute_bucket_size);
                    println!("note_directory={}", config.note_directory);
                }
            }
        }
    }
    Ok(())
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

    #[test]
    fn invalid_hour() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 24:00");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("invalid hour input"));
        Ok(())
    }

    #[test]
    fn invalid_hour_for_meridian() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 13:00am");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("invalid hour input for meridiem time"));
        Ok(())
    }
}
