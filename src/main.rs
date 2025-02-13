use std::fmt;
use std::io::Read;

use anyhow::Context;

use user_store::DataStore;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Read,
    Write,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Read => "read",
            Self::Write => "write",
        };
        f.write_str(s)
    }
}

fn main() -> anyhow::Result<()> {
    let base = std::env::var("BASE_DIR").context("Failed to read BASE_DIR")?;
    let store = DataStore::new(base).context("Failed to initialize data store")?;
    let args: Vec<_> = std::env::args().collect();
    let proc = &args[0];
    let operation = args
        .get(1)
        .with_context(|| format!("Usage: {proc} [read|write] <entry>"))?;
    let operation = match operation.as_str() {
        "read" => Operation::Read,
        "write" => Operation::Write,
        _ => anyhow::bail!("Usage: {proc} [read|write] <entry>"),
    };
    let entry = args
        .get(2)
        .with_context(|| format!("Usage: {proc} {operation} <entry>"))?;
    let entry = store.parse_entry(entry)?;
    match operation {
        Operation::Read => {
            let data = store.read(&entry)?;
            let data = serde_json::to_string(&data).context("Failed to serialize read data")?;
            println!("{data}");
        }
        Operation::Write => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read data from stdin")?;
            let data = serde_json::from_str(&buf).context("Failed to deserialize data")?;
            store.write(&entry, data)?;
        }
    }
    Ok(())
}
