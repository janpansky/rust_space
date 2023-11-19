use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt};
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use log::{info};

extern crate shared_library;

use shared_library::{MessageType, create_directories};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Create directories if they don't exist
    create_directories()?;

    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:11111").await?;
    info!("Server listening on 0.0.0.0:11111");

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_client(socket));
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream) {
    info!("Client connected");

    // Handle incoming messages from clients
    let mut buffer = vec![0u8; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        info!("{:?}", n);

        if n == 0 {
            break;
        }

        if let Ok(message) = serde_cbor::from_slice::<MessageType>(&buffer[0..n]) {
            match message {
                MessageType::File(filename, file_content) => {
                    // Handle file transfer
                    let file_path = format!("files/{}", filename);
                    save_file(&file_path, &file_content).unwrap();
                    info!("Received file: {}", filename);
                }
                MessageType::Image(image_content) => {
                    // Handle image transfer
                    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
                    let filename = format!("images/{}.png", timestamp);
                    save_file(&filename, &image_content).unwrap();
                    info!("Received image: {}", filename);
                }
                MessageType::Text(text) => {
                    // Handle text message
                    info!("Received text: {}", text);
                }
                MessageType::Quit => {
                    return; // Terminate the client connection
                }
            }
        }
    }
}

// Define a function to save a file
fn save_file(file_path: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;
    Ok(())
}