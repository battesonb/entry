use chrono::prelude::{DateTime, Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::BufReader,
    str::FromStr,
};

use crate::{config::Config, errors::EntryError};

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaCount {
    One,
    Many,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaDataType {
    String,
    Number,
    Date,
}

impl FromStr for SchemaDataType {
    type Err = EntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "date" => Ok(SchemaDataType::Date),
            "number" => Ok(SchemaDataType::Number),
            "string" => Ok(SchemaDataType::String),
            _ => Err(EntryError::SchemaParseError),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaType {
    pub count: SchemaCount,
    pub data_type: SchemaDataType,
}

#[derive(Deserialize, Serialize)]
pub struct Schema {
    shape: HashMap<String, SchemaType>,
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            shape: HashMap::new(),
        }
    }
}

impl Schema {
    pub fn insert(&mut self, key: String, value: SchemaType) -> () {
        self.shape.insert(key, value);
    }

    pub fn is_empty(&self) -> bool {
        return self.shape.is_empty();
    }

    pub fn list(config: &Config) -> Vec<String> {
        if let Ok(dir) = fs::read_dir(format!("{}/schema", &config.data_directory)) {
            let names: Vec<String> = dir
                .filter_map(|f| {
                    if let Ok(entry) = f {
                        let file_name = entry.file_name().to_string_lossy().to_string();
                        if file_name.ends_with(".json") {
                            Some(file_name[..file_name.len() - 5].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            return names;
        }
        return Vec::new();
    }

    pub fn print(&self) -> () {
        if let Ok(json_str) = serde_json::to_string(&self) {
            println!("{}", json_str);
        }
    }

    pub fn save(&self, path: &str, name: &str) -> Result<(), EntryError> {
        let full_path = format!("{}/{}.json", path, name);
        let result = OpenOptions::new().create(true).write(true).open(full_path);
        match result {
            Ok(file) => {
                let write_result = serde_json::to_writer_pretty(file, &self);
                match write_result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(EntryError::SchemaSaveError),
                }
            }
            Err(_) => Err(EntryError::SchemaSaveError),
        }
    }

    pub fn load(path: &str, name: &str) -> Result<Schema, EntryError> {
        let full_path = format!("{}/{}.json", path, name);
        let result = File::open(full_path);
        return match result {
            Ok(file) => {
                let reader = BufReader::new(file);
                let schema_result = serde_json::from_reader(reader);
                match schema_result {
                    Ok(schema) => Ok(schema),
                    Err(_) => Err(EntryError::SchemaParseError),
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
