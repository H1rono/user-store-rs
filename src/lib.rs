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

impl DataStore {
    pub fn new(base: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let path = base.as_ref().canonicalize()?;
        // assume path is directory
        let slf = Self { base: path };
        Ok(slf)
    }

    pub fn read(&self, entry: &str) -> anyhow::Result<User> {
        let filename = self.base.join(entry);
        let content =
            std::fs::read_to_string(filename).context("Failed to read from filesystem")?;
        let data =
            serde_json::from_str(&content).context("Failed to deserialize data from filesystem")?;
        Ok(data)
    }

    pub fn write(&self, entry: &str, data: User) -> anyhow::Result<()> {
        let filename = self.base.join(entry);
        let content = serde_json::to_string(&data).context("Failed to serialize data")?;
        std::fs::write(filename, content)
            .context("Failed to write serialized data to filesystem")?;
        Ok(())
    }
}
