#[derive(Debug)]
pub enum EntryError {
    SchemaLoadError,
    SchemaParseError,
    SchemaRemoveError,
    SchemaSaveError,
}
