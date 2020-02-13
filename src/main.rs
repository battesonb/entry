mod args;
mod utils;

use args::{Command, Config, Entry};
use utils::{get_datetime, get_directory, has_text, prompt_opt};

use exitfailure::ExitFailure;
use failure::{ResultExt};
use std::{cmp, env::{var}, fs, io::{BufRead, BufReader}};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let config: Config = confy::load("entry")?;
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
            let editor = var("EDITOR")?;
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
        Command::Setup {} => {
            println!("[1] Create file on call to the `new` subcommand (y/n)? ({})", if config.create_file { "y" } else { "n" });
            let create_file_input: Option<String> = prompt_opt();
            let create_file: bool = match create_file_input {
                Some(v) => if v == "y" { true } else { false },
                _ => config.create_file
            };

            println!("[2] Default note name? ({})", config.default_note_name);
            let default_note_name_input: Option<String> = prompt_opt();
            let default_note_name: String = match default_note_name_input {
                Some(dnn) => dnn,
                _ => config.default_note_name
            };

            println!("[3] Minute bucket size (1-60)? ({})", config.minute_bucket_size);
            let minute_bucket_size_input: Option<String> = prompt_opt();
            let minute_bucket_size: u32 = match minute_bucket_size_input {
                Some(m) => {
                    let minute_bucket_size_u32: Result<u32, _> = m.parse();
                    match minute_bucket_size_u32 {
                        Ok(v) => cmp::max(1, cmp::min(60, v)),
                        _ => config.minute_bucket_size
                    }
                },
                _ => config.minute_bucket_size
            };

            println!("[4] Note directory ({})", config.note_directory);
            let note_directory_input: Option<String> = prompt_opt();
            let note_directory = match note_directory_input {
                Some(nd) => nd,
                _ => config.note_directory
            };

            confy::store("entry", Config {
                create_file: create_file,
                default_note_name: default_note_name,
                minute_bucket_size: minute_bucket_size,
                note_directory: note_directory
            })?;
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
