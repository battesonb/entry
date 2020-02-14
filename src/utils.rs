use chrono::prelude::{Datelike, DateTime, Local, Timelike};
use failure::{bail, Error};
use regex::Regex;
use std::{env::{var}, fs, io::{BufReader, Lines}};

pub fn get_directory(note_directory: &str, entry_name: &str) -> String {
    return format!("{}/{}", shellexpand::tilde(&note_directory), entry_name);
}

pub fn get_datetime(time: &str, minute_bucket_size: u32) -> Result<DateTime<Local>, Error> {
    let now = Local::now();

    match time {
        "now" => Ok(now.with_minute((now.minute() / minute_bucket_size) * minute_bucket_size).unwrap()),
        "tomorrow" => Ok(now.with_day(now.day() + 1).unwrap().with_minute((now.minute() / minute_bucket_size) * minute_bucket_size).unwrap()),
        input => {
            let re = Regex::new("(\\d+(?::?\\d+))(am|pm)?").unwrap();
            let captures = match re.captures(input) {
                Some(payload) => payload,
                None => bail!("unrecognized time input")
            };
            let hour_and_minute = captures.get(1).unwrap().as_str();
            let split = hour_and_minute.split(':').collect::<Vec<&str>>();
            let hour_digits = split.get(0).unwrap().parse::<u32>().unwrap();
            let meridiem = match captures.get(2) {
                Some(value) => value.as_str(),
                None => ""
            };
            let is_meridiem_time = meridiem == "pm" || meridiem == "am";
            let is_pm = meridiem == "pm";
            let minute: u32 = if split.len() == 2 { split.get(1).unwrap().parse().unwrap() } else { 0 };

            if is_meridiem_time && hour_digits > 12 {
                bail!("invalid hour input for meridiem time");
            }
            if !is_meridiem_time && hour_digits > 23 {
                bail!("invalid hour input")
            }
            if minute > 59 {
                bail!("invalid minute input")
            }

            let hour: u32 = if is_meridiem_time {
                if is_pm {
                    if hour_digits == 12 {
                        12
                    } else {
                        hour_digits + 12
                    }
                } else {
                    hour_digits % 12
                }
            } else {
                hour_digits
            };

            let is_tomorrow = hour < now.hour() || (hour == now.hour() && minute < now.minute());

            Ok(now
                .with_day(now.day() + if is_tomorrow { 1 } else { 0 }).unwrap()
                .with_hour(hour).unwrap()
                .with_minute((minute / minute_bucket_size) * minute_bucket_size).unwrap()
                .with_second(0).unwrap()
                .with_nanosecond(0).unwrap()
            )
        }
    }
}

pub fn get_editor() -> Result<String, Error> {
    let editor_env: Result<String, _> = var("EDITOR");
    match editor_env {
        Ok(editor) => {
            return Ok(editor);
        },
        _ => {
            let vim_result = std::process::Command::new("which")
                                                   .arg("vim")
                                                   .output();
            match vim_result {
                Ok(output) => {
                    if output.status.success() {
                        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
                    } else {
                        bail!("failed to determine editor");
                    }
                },
                _ => {
                    bail!("failed to determine editor");
                }
            }
        }
    }
}

pub fn has_text(text: &str, lines: Lines<BufReader<fs::File>>) -> bool {
    for line in lines {
        if line.unwrap().contains(text) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_directory_appends_entry_name_to_directory() {
    let directory = get_directory("/a/directory", "note");
    assert_eq!(directory, "/a/directory/note");
  }
}
