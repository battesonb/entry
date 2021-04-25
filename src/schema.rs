use chrono::prelude::{DateTime, Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::{
    collections::{btree_map::IntoIter, BTreeMap},
    fs::{self, File, OpenOptions},
    io::BufReader,
    str::FromStr,
};

use crate::{config::Config, errors::EntryError};

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaCount {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "many")]
    Many,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SchemaDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "date")]
    Date,
}

impl std::fmt::Display for SchemaDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_args!("{:?}", self).to_string().to_lowercase())
    }
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

impl std::fmt::Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.count {
            SchemaCount::One => f.write_fmt(format_args!("{}", self.data_type)),
            SchemaCount::Many => f.write_fmt(format_args!("array of {}s", self.data_type)),
        }
    }
}

impl SchemaType {
    pub fn parse(&self, value: &str) -> Option<Value> {
        let trimmed = value.trim();
        if trimmed.len() == 0 {
            return None;
        }
        match self.count {
            SchemaCount::Many => {
                let split = trimmed.split(",");
                let mut vec: Vec<Value> = Vec::new();
                for v in split {
                    if let Some(parsed) = self.parse_individual(v) {
                        vec.push(parsed);
                    } else {
                        return None;
                    }
                }
                Some(Value::Array(vec))
            }
            SchemaCount::One => self.parse_individual(trimmed),
        }
    }

    fn parse_individual(&self, value: &str) -> Option<Value> {
        match self.data_type {
            SchemaDataType::Date => {
                let custom_formats = [
                    "%Y-%m-%d",
                    "%Y/%m/%d",
                    "%Y-%m-%d %H:%M:%S",
                    "%Y/%m/%d %H:%M:%S",
                ];

                for &format in custom_formats.iter() {
                    let result = DateTime::parse_from_str(value, format);
                    if let Ok(_) = result {
                        return Some(Value::String(value.to_string()));
                    }
                }
                None
            }
            SchemaDataType::Number => Number::from_str(value).map(|v| Value::Number(v)).ok(),
            SchemaDataType::String => Some(Value::String(value.to_string())),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Schema {
    shape: BTreeMap<String, SchemaType>,
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            shape: BTreeMap::new(),
        }
    }
}

impl IntoIterator for Schema {
    type Item = (String, SchemaType);
    type IntoIter = IntoIter<String, SchemaType>;

    fn into_iter(self) -> Self::IntoIter {
        self.shape.into_iter()
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
        let result = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(full_path);
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
