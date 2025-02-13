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

fn main() -> anyhow::Result<Result<(), String>> {
    let base = std::env::var("BASE_DIR").context("Failed to read BASE_DIR")?;
    let store = DataStore::new(base).context("Failed to initialize data store")?;
    let args: Vec<_> = std::env::args().collect();
    let proc = &args[0];
    let usage = || format!("Usage: {proc} [read|write] <entry>");
    let Some(operation) = args.get(1) else {
        return Ok(Err(usage()));
    };
    let operation = match operation.as_str() {
        "read" => Operation::Read,
        "write" => Operation::Write,
        _ => return Ok(Err(usage())),
    };
    let Some(entry) = args.get(2) else {
        return Ok(Err(format!("Usage: {proc} {operation} <entry>")));
    };
    match operation {
        Operation::Read => {
            let data = match store.read(entry)? {
                Ok(d) => d,
                Err(e) => return Ok(Err(e)),
            };
            let data = serde_json::to_string(&data).context("Failed to serialize read data")?;
            println!("{data}");
        }
        Operation::Write => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("Failed to read data from stdin")?;
            let data = match serde_json::from_str(&buf) {
                Ok(d) => d,
                Err(e) => return Ok(Err(format!("Received invalid data: {e}"))),
            };
            match store.write(entry, data)? {
                Ok(()) => {}
                Err(e) => return Ok(Err(e)),
            };
        }
    }
    Ok(Ok(()))
}
