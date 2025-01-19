use anyhow::Error as AnyhowError;
use std::fs;

fn read_file(path: &str) -> Result<String, AnyhowError> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}
