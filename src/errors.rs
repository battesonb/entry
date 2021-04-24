#[derive(Debug)]
pub enum EntryError {
    ConfigInvalidKey,
    SchemaLoadError,
    SchemaParseError,
    SchemaSaveError,
}
