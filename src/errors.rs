#[derive(Debug)]
pub enum EntryError {
    SchemaLoadError,
    SchemaParseError,
    SchemaSaveError,
}
