use chrono::prelude::{DateTime, Datelike, Local, Timelike};
use failure::{bail, Error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader, Lines},
};

use crate::errors::EntryError;

#[derive(Deserialize, Serialize)]
enum SchemaCount {
    One,
    Many,
}

#[derive(Deserialize, Serialize)]
enum SchemaDataType {
    String,
    Number,
    Date,
}

#[derive(Deserialize, Serialize)]
struct SchemaType {
    count: SchemaCount,
    data_type: SchemaDataType,
}

#[derive(Deserialize, Serialize)]
pub struct Schema {
    shape: HashMap<String, SchemaType>,
}

impl Schema {
    pub fn save(&self) -> Result<(), EntryError> {
        todo!("Scema#save");
    }

    pub fn load(path: &str) -> Result<Schema, EntryError> {
        let result = File::open(path);
        return match result {
            Ok(file) => {
                let reader = BufReader::new(file);
                let schema_result = serde_json::from_reader(reader);
                match schema_result {
                    Ok(schema) => Ok(schema),
                    Err(error) => Err(EntryError::SchemaParseError),
                }
            }
            Err(_) => Err(EntryError::SchemaLoadError),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_directory_appends_entry_name_to_directory() {
        // let directory = Schema::get_directory("/a/directory", "note");
        // assert_eq!(directory, "/a/directory/note");
    }
}
