#[derive(Debug)]
pub enum SchemaError {
    LoadError,
    ParseError,
    RemoveError,
    SaveError,
}
