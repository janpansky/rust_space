use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    File(String, Vec<u8>),
    Image(Vec<u8>),
    Text(String),
    Quit,
}