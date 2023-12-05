//! # Robot Dreams Client
//!
//! This module contains the client-side logic for the Robot Dreams application,
//! which connects to the server, handles user input, and sends messages (text,
//! files, images) to the server. The client uses Tokio for asynchronous runtime,
//! Serde for message serialization, and various other libraries for image processing,
//! logging, and shared functionality with the server.
//!
//! ## Modules
//!
//! - [`main`]: Contains the main entry point for the client application.
//! - [`send_quit_message`]: Sends a quit message to the server, terminating the client connection.
//! - [`process_input`]: Processes user input based on the command, handling text, file, or image messages.
//! - [`handle_file_message`]: Handles file messages, sending file content to the server.
//! - [`handle_image_message`]: Handles image messages, sending image content to the server.
//! - [`handle_text_message`]: Handles text messages, sending text content to the server.
//! - [`is_logged_in`]: Checks if the user is logged in, handling the login process with the server.
//! - [`receive_login_response`]: Receives and processes the login response from the server.
//!
//! ## Usage
//!
//! To run the client, execute the binary produced by the compilation process.
//!
//! ```
//! cargo run --bin client
//! ```

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

use image::ImageFormat;
use log::info;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::time::sleep;

extern crate shared_library;

use shared_library::{MessageType, create_directories};

const BUFFER_SIZE: usize = 16384;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Create directories if they don't exist
    create_directories()?;

    // Initialize the logging formatter
    tracing_subscriber::fmt::init();

    // Connect to the server
    let server_addr = "0.0.0.0:11111";

    loop {
        match TcpStream::connect(server_addr).await {
            Ok(mut stream) => {
                info!("Connected to server at {}", server_addr);

                // Set up standard input reader for user input
                let stdin = io::stdin();
                let mut reader = io::BufReader::new(stdin);

                // Login
                if !is_logged_in(&mut stream).await? {
                    eprintln!("Login failed. Exiting...");
                    send_quit_message(&mut stream).await?;
                    info!("Quit message sent. Client connection ended.");
                    return Ok(());
                }
                // Create directories for images and files
                let images_dir = "images";
                let files_dir = "files";
                std::fs::create_dir_all(images_dir)?;
                std::fs::create_dir_all(files_dir)?;

                // Main loop for user input
                loop {
                    let mut input = String::new();
                    println!("Enter a message (or type '.quit' to exit):");

                    // Use the AsyncBufReadExt trait for asynchronous reading
                    reader.read_line(&mut input).await?;

                    if input.trim() == ".quit" {
                        send_quit_message(&mut stream).await?;
                        info!("Quit message sent. Client connection ended.");
                        return Ok(());
                    } else {
                        process_input(&mut stream, &input).await?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error connecting to the server: {}. Retrying in 5 seconds...", e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

// Send a quit message to the server
pub async fn send_quit_message(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let quit_message = MessageType::Quit;
    let message_bytes = serde_cbor::to_vec(&quit_message)?;
    stream.write_all(&message_bytes).await?;
    Ok(())
}

// Process user input based on the command
pub async fn process_input(
    stream: &mut TcpStream,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    if input.starts_with(".file") {
        handle_file_message(stream, input).await?;
    } else if input.starts_with(".image") {
        handle_image_message(stream, input).await?;
    } else {
        handle_text_message(stream, input).await?;
    }

    Ok(())
}

// Handle the file message
pub async fn handle_file_message(
    stream: &mut TcpStream,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let filename = input.trim_start_matches(".file ").trim();

    // Construct the file path
    let file_path = format!("assets/files/{}", filename);
    info!("Filepath: {}", file_path);

    // Read the file content from the file
    let mut file_content = Vec::new();
    match File::open(&file_path) {
        Ok(mut file) => {
            file.read_to_end(&mut file_content)?;
            info!("File Content Length: {}", file_content.len());
        }
        Err(e) => {
            // Handle the file not found error
            eprintln!("Error opening file '{}': {}", filename, e);
            return Ok(());
        }
    }

    // Write the file content to the stream
    let file_message = MessageType::File(filename.to_string(), file_content);
    let message_bytes = serde_cbor::to_vec(&file_message)?;
    stream.write_all(&message_bytes).await?;

    info!("File '{}' sent successfully.", filename);

    Ok(())
}

// Handle the image message
pub async fn handle_image_message(
    stream: &mut TcpStream,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let filename = input.trim_start_matches(".image ").trim();

    // Construct the file path
    let image_path = format!("assets/images/{}", filename);
    info!("Filepath: {}", image_path);

    // Read the image assets from the file
    let mut image_content = Vec::new();
    match File::open(&image_path) {
        Ok(mut file) => {
            file.read_to_end(&mut image_content)?;
            info!("Image Content Length: {}", image_content.len());
        }
        Err(e) => {
            // Handle the file not found error
            if let Some(errno) = e.raw_os_error() {
                if errno == 2 {
                    // No such file or directory error
                    eprintln!("Error opening image file '{}': {}", filename, e);
                    return Ok(());
                }
            }
            // For other errors, print the error message and optionally return an error
            eprintln!("Error opening image file '{}': {}", filename, e);
            return Err(Box::new(e));
        }
    }

    // Convert the image to PNG format (bonus challenge)
    let dynamic_image = image::load_from_memory_with_format(&image_content, ImageFormat::Png)?;

    // Convert the DynamicImage to a vector of bytes
    let mut image_bytes = Vec::new();
    dynamic_image.write_to(&mut image_bytes, ImageFormat::Png)?;

    // Use the original vector for the MessageType::Image variant
    let image_message = MessageType::Image(filename.to_string(), image_content);
    let message_bytes = serde_cbor::to_vec(&image_message)?;

    // Attempt to write to the stream
    if let Err(e) = stream.write_all(&message_bytes).await {
        // Handle the broken pipe error
        if e.kind() == std::io::ErrorKind::BrokenPipe {
            eprintln!("Server closed the connection. Exiting...");
            std::process::exit(1);
        } else {
            return Err(Box::new(e));
        }
    }

    info!("Image '{}' sent successfully.", filename);

    Ok(())
}

// Handle the text message
pub async fn handle_text_message(
    stream: &mut TcpStream,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let text_message = MessageType::Text(input.to_string());
    let message_bytes = serde_cbor::to_vec(&text_message)?;
    stream.write_all(&message_bytes).await?;
    Ok(())
}

// Check if the user is logged in
async fn is_logged_in(stream: &mut TcpStream) -> Result<bool, Box<dyn Error>> {
    // Initialize the reader within the function
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    // You can send a specific message to the server and wait for a response
    // or use any other authentication mechanism you have in place.
    println!("Enter your username:");
    let mut username = String::new();
    reader.read_line(&mut username).await?;

    println!("Enter your password:");
    let mut password = String::new();
    reader.read_line(&mut password).await?;

    // Send login message to the server
    let login_message = MessageType::Login(username.trim().to_string(), password.trim().to_string());
    let message_bytes = serde_cbor::to_vec(&login_message)?;
    stream.write_all(&message_bytes).await?;

    // Wait for the server response
    let response_message = receive_login_response(stream).await;

    match response_message {
        Ok(true) => {
            println!("Login successful!");
            Ok(true)
        }
        Ok(false) => {
            eprintln!("Login failed.");
            Ok(false)
        }
        Err(err) => {
            eprintln!("Error receiving login response: {:?}", err);
            Ok(false)
        }
    }
}

async fn receive_login_response(stream: &mut TcpStream) -> Result<bool, Box<dyn Error>> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let n = stream.read(&mut buffer).await?;

    // Deserialize the entire MessageType
    let response_message: MessageType = serde_cbor::from_slice(&buffer[..n])?;

    // Match on the specific variant
    match response_message {
        MessageType::LoginResponse(login_successful) => Ok(login_successful),
        _ => {
            eprintln!("Unexpected response from the server.");
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    // Import the client module
    use super::*;

    #[tokio::test]
    async fn test_handle_text_message() {
        let mut stream = TcpStream::connect("127.0.0.1:11111").await.unwrap();
        let input = "Hello, server!";
        handle_text_message(&mut stream, input).await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_file_message() {
        let mut stream = TcpStream::connect("127.0.0.1:11111").await.unwrap();
        let input = ".file example.txt";
        handle_file_message(&mut stream, input).await.unwrap();
    }

    #[tokio::test]
    async fn test_handle_image_message() {
        // Create a TcpStream for testing
        let mut stream = TcpStream::connect("127.0.0.1:11111").await.unwrap();

        let input = ".image rust.jpg";
        handle_image_message(&mut stream, input).await.unwrap();
    }
}