use anyhow::Context;
use serde::{Deserialize, Serialize};

pub mod error;

pub use error::Failure;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: u16,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    base: std::path::PathBuf,
}

struct ValidEntry<'a>(std::borrow::Cow<'a, str>);

impl DataStore {
    pub fn new(base: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let path = base.as_ref().canonicalize()?;
        // assume path is directory
        let slf = Self { base: path };
        Ok(slf)
    }

    fn parse_entry<'a>(&self, entry: &'a str) -> Option<ValidEntry<'a>> {
        use std::path::{Component, Path};

        let path_components = Path::new(entry).components().collect::<Vec<_>>();
        let entry = match path_components.as_slice() {
            [Component::Normal(entry)] => entry,
            _ => return None,
        };
        let entry = entry.to_str().unwrap();
        Some(ValidEntry(entry.into()))
    }

    pub fn read(&self, entry: &str) -> anyhow::Result<Result<User, String>> {
        let Some(entry) = self.parse_entry(entry) else {
            return Ok(Err("Received invalid entry".to_string()));
        };
        let filename = self.base.join(&*entry.0);
        let exists = filename.try_exists().context("Failed to search entry")?;
        if !exists {
            return Ok(Err("Entry not found".to_string()));
        }
        let content =
            std::fs::read_to_string(filename).context("Failed to read from filesystem")?;
        let data =
            serde_json::from_str(&content).context("Failed to deserialize data from filesystem")?;
        Ok(Ok(data))
    }

    pub fn write(&self, entry: &str, data: User) -> anyhow::Result<Result<(), String>> {
        let Some(entry) = self.parse_entry(entry) else {
            return Ok(Err("Received invalid entry".to_string()));
        };
        let filename = self.base.join(&*entry.0);
        let content = serde_json::to_string(&data).context("Failed to serialize data")?;
        std::fs::write(filename, content)
            .context("Failed to write serialized data to filesystem")?;
        Ok(Ok(()))
    }
}
