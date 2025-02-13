use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub age: u16,
}

#[derive(Debug, Clone)]
pub struct DataStore {
    base: std::path::PathBuf,
}

pub struct ValidEntry<'a>(std::borrow::Cow<'a, str>);

impl DataStore {
    pub fn new(base: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let path = base.as_ref().canonicalize()?;
        // assume path is directory
        let slf = Self { base: path };
        Ok(slf)
    }

    pub fn parse_entry<'a>(&self, entry: &'a str) -> anyhow::Result<ValidEntry<'a>> {
        use std::path::{Component, Path};

        let path_components = Path::new(entry).components().collect::<Vec<_>>();
        let entry = match path_components.as_slice() {
            [Component::Normal(entry)] => entry,
            _ => anyhow::bail!("Received invalid entry name"),
        };
        let entry = entry.to_str().unwrap();
        Ok(ValidEntry(entry.into()))
    }

    pub fn read(&self, entry: &ValidEntry<'_>) -> anyhow::Result<User> {
        let filename = self.base.join(&*entry.0);
        let content =
            std::fs::read_to_string(filename).context("Failed to read from filesystem")?;
        let data =
            serde_json::from_str(&content).context("Failed to deserialize data from filesystem")?;
        Ok(data)
    }

    pub fn write(&self, entry: &ValidEntry<'_>, data: User) -> anyhow::Result<()> {
        let filename = self.base.join(&*entry.0);
        let content = serde_json::to_string(&data).context("Failed to serialize data")?;
        std::fs::write(filename, content)
            .context("Failed to write serialized data to filesystem")?;
        Ok(())
    }
}
