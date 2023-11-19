use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    File(String, Vec<u8>),
    Image(Vec<u8>),
    Text(String),
    Quit,
}

pub fn create_directories() -> Result<(), std::io::Error> {
    // Create directories if they don't exist
    fs::create_dir_all("files")?;
    fs::create_dir_all("images")?;

    Ok(())
}