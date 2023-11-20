use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

use image::{ImageFormat};
use log::info;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

extern crate shared_library;

use shared_library::{MessageType, create_directories};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Create directories if they don't exist
    create_directories()?;

    // Initialize the logging formatter
    tracing_subscriber::fmt::init();

    // Connect to the server
    let server_addr = "0.0.0.0:11111";
    let mut stream = TcpStream::connect(server_addr).await?;
    info!("Connected to server at {}", server_addr);

    // Set up standard input reader for user input
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    // Create directories for images and files
    let images_dir = "images";
    let files_dir = "files";
    std::fs::create_dir_all(images_dir)?;
    std::fs::create_dir_all(files_dir)?;

    // Main loop for user input
    loop {
        let mut input = String::new();
        println!("Enter a message (or type '.quit' to exit):");
        reader.read_line(&mut input).await?;

        if input.trim() == ".quit" {
            send_quit_message(&mut stream).await?;
            break;
        } else {
            process_input(&mut stream, &input, files_dir).await?;
        }
    }

    Ok(())
}

// Send a quit message to the server
async fn send_quit_message(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let quit_message = MessageType::Quit;
    let message_bytes = serde_cbor::to_vec(&quit_message)?;
    stream.write_all(&message_bytes).await?;
    Ok(())
}

// Process user input based on the command
async fn process_input(
    stream: &mut TcpStream,
    input: &str,
    files_dir: &str,
) -> Result<(), Box<dyn Error>> {
    if input.starts_with(".file") {
        handle_file_message(stream, input, files_dir).await?;
    } else if input.starts_with(".image") {
        handle_image_message(stream).await?;
    } else {
        handle_text_message(stream, input).await?;
    }

    Ok(())
}

// Handle the image message
async fn handle_file_message(
    stream: &mut TcpStream,
    input: &str,
    files_dir: &str,
) -> Result<(), Box<dyn Error>> {
    let filename = input.trim_start_matches(".file ").trim();
    let file_content = Vec::new(); // Read file content from the file

    let file_path = format!("{}/{}", files_dir, filename);
    info!("filepath: {}", file_path);
    let mut file = File::create(&file_path)?;
    file.write_all(&file_content)?;

    info!("Sending file: {}", filename);

    let file_message = MessageType::File(filename.to_string(), file_content);
    let message_bytes = serde_cbor::to_vec(&file_message)?;
    stream.write_all(&message_bytes).await?;

    Ok(())
}

async fn handle_image_message(
    stream: &mut TcpStream,
) -> Result<(), Box<dyn Error>> {
    // Assuming you have a file with image content, change the path accordingly
    let file_path = "content/images/rust.png";

    // Read the image content from the file
    let mut image_content = Vec::new();
    File::open(file_path)?.read_to_end(&mut image_content)?;

    // Print or log the image content for debugging
    info!("Image Content Length: {}", image_content.len());

    // Convert the image to PNG format (bonus challenge)
    let dynamic_image = image::load_from_memory_with_format(&image_content, ImageFormat::Png)?;

    // Convert the DynamicImage to a vector of bytes
    let mut image_bytes = Vec::new();
    dynamic_image.write_to(&mut image_bytes, ImageFormat::Png)?;

    // Use the original vector for the MessageType::Image variant
    let image_message = MessageType::Image(image_content);
    let message_bytes = serde_cbor::to_vec(&image_message)?;
    stream.write_all(&message_bytes).await?;
    info!("Sending image...");

    Ok(())
}

// Handle the text message
async fn handle_text_message(
    stream: &mut TcpStream,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let text_message = MessageType::Text(input.to_string());
    let message_bytes = serde_cbor::to_vec(&text_message)?;
    stream.write_all(&message_bytes).await?;
    Ok(())
}