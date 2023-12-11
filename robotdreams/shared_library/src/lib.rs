//! # Shared Library
//!
//! This module contains shared functionality and data structures used by both the
//! Robot Dreams server and client applications. It includes a custom message type for
//! communication, along with functions for message serialization/deserialization and
//! directory creation.
//!
//! ## Modules
//!
//! - [`MessageType`]: Enum representing different types of messages exchanged between server and client.
//! - [`create_directories`]: Function to create necessary directories for file and image storage.
//!
//! ## Usage
//!
//! Include this module in both the server and client applications for shared functionality.
//!
//! ```
//! use shared_library::{MessageType, create_directories};
//! ```

use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    File(String, Vec<u8>),
    Image(String, Vec<u8>),
    Text(String),
    Login(String, String),
    LoginResponse(bool),
    Quit,
}

/// Creates necessary directories for file and image storage.
pub fn create_directories() -> Result<(), std::io::Error> {
    // Create directories if they don't exist
    fs::create_dir_all("files")?;
    fs::create_dir_all("images")?;

    Ok(())
}
