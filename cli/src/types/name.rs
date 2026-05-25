use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NameError {}

#[derive(Debug, Clone, Serialize)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn from_str(name: &str) -> Result<Name, NameError> {
        return Ok(Name {
            name: name.to_string(),
        });
    }
}
