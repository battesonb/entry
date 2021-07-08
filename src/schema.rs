use chrono::{
    prelude::{NaiveDate, NaiveDateTime},
    DateTime,
};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::{
    collections::{btree_map::IntoIter, BTreeMap},
    fs::{self, File, OpenOptions},
    io::BufReader,
    str::FromStr,
};

use crate::{config::Config, errors::SchemaError};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum SchemaCount {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "many")]
    Many,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum SchemaDataType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "datetime")]
    DateTime,
}

impl std::fmt::Display for SchemaDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_args!("{:?}", self).to_string().to_lowercase())
    }
}

impl FromStr for SchemaDataType {
    type Err = SchemaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "date" => Ok(SchemaDataType::Date),
            "number" => Ok(SchemaDataType::Number),
            "string" => Ok(SchemaDataType::String),
            _ => Err(SchemaError::ParseError),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
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
        if trimmed.is_empty() {
            return match self.count {
                SchemaCount::Many => Some(Value::Array(vec![])),
                SchemaCount::One => None,
            };
        }
        match self.count {
            SchemaCount::Many => {
                let split = trimmed.split(',');
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
                let custom_date_formats = ["%Y-%m-%d", "%Y/%m/%d"];

                for &format in custom_date_formats.iter() {
                    if NaiveDate::parse_from_str(value, format).is_ok() {
                        return Some(Value::String(value.to_string()));
                    }
                }
                None
            }
            SchemaDataType::DateTime => {
                if DateTime::parse_from_rfc2822(value).is_ok() {
                    return Some(Value::String(value.to_string()));
                }

                if DateTime::parse_from_rfc3339(value).is_ok() {
                    return Some(Value::String(value.to_string()));
                }

                let custom_datetime_formats = [
                    "%Y-%m-%d %H:%M:%S",
                    "%Y/%m/%d %H:%M:%S",
                    "%Y-%m-%d %H:%M",
                    "%Y/%m/%d %H:%M",
                ];

                for &format in custom_datetime_formats.iter() {
                    if NaiveDateTime::parse_from_str(value, format).is_ok() {
                        return Some(Value::String(value.to_string()));
                    }
                }
                None
            }
            SchemaDataType::Number => Number::from_str(value).map(Value::Number).ok(),
            SchemaDataType::String => Some(Value::String(value.to_string())),
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
    pub fn insert(&mut self, key: String, value: SchemaType) {
        self.shape.insert(key, value);
    }

    pub fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    pub fn list(config: &Config) -> Vec<String> {
        if let Ok(dir) = fs::read_dir(format!("{}/schema", &config.data_directory())) {
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
        Vec::new()
    }

    pub fn print(&self) {
        if let Ok(json_str) = serde_json::to_string(&self) {
            println!("{}", json_str);
        }
    }

    pub fn save(&self, path: &str, name: &str) -> Result<(), SchemaError> {
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
                    Err(_) => Err(SchemaError::SaveError),
                }
            }
            Err(_) => Err(SchemaError::SaveError),
        }
    }

    pub fn load(path: &str, name: &str) -> Result<Schema, SchemaError> {
        let full_path = format!("{}/{}.json", path, name);
        let result = File::open(full_path);
        match result {
            Ok(file) => {
                let reader = BufReader::new(file);
                let schema_result = serde_json::from_reader(reader);
                match schema_result {
                    Ok(schema) => Ok(schema),
                    Err(_) => Err(SchemaError::ParseError),
                }
            }
            Err(_) => Err(SchemaError::LoadError),
        }
    }

    pub fn remove(path: &str, name: &str) -> Result<(), SchemaError> {
        let full_path = format!("{}/{}.json", path, name);
        match fs::remove_file(full_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(SchemaError::RemoveError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_types_can_parse_individual_data_types() {
        let data_types = [
            (
                SchemaDataType::Date,
                "2021/04/25",
                Value::String("2021/04/25".to_string()),
            ),
            (
                SchemaDataType::DateTime,
                "2021/04/25 11:17:00",
                Value::String("2021/04/25 11:17:00".to_string()),
            ),
            (
                SchemaDataType::Number,
                "20.5",
                Value::Number(Number::from_f64(20.5).unwrap()),
            ),
            (
                SchemaDataType::String,
                "anything531",
                Value::String("anything531".to_string()),
            ),
        ];

        for (data_type, input, value) in data_types.iter() {
            let schema_type = SchemaType {
                count: SchemaCount::One,
                data_type: *data_type,
            };
            assert_eq!(schema_type.parse(*input).unwrap(), value.clone());
        }
    }
}
