use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
enum MessageType {
    File(String),
    Image(String),
    Text(String),
    Quit,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:11111").await?;
    println!("Server listening on 0.0.0.0:11111");

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_client(socket));
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream) {
    println!("Client connected");

    // Handle incoming messages from clients here
    let mut buffer = vec![0u8; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        println!("{:?}", n);

        if n == 0 {
            break;
        }

        if let Ok(message) = serde_cbor::from_slice::<MessageType>(&buffer[0..n]) {
            match message {
                MessageType::File(path) => {
                    if let Ok(n) = socket.read(&mut buffer).await {
                        if n == 0 {
                            return;
                        }
                        // Assuming the entire file content is received in one go
                        let file_content = &buffer[0..n];

                        // Save the file to the specified path
                        if let Err(e) = save_file(&path, file_content) {
                            // Handle the error (e.g., log or send an error message back to the client)
                            println!("Error saving file: {}", e);
                        }
                    }
                }
                MessageType::Image(_path) => {
                    // Handle image transfer
                }
                MessageType::Text(_text) => {
                    // Handle text message
                }
                MessageType::Quit => {
                    // Handle client quitting
                    return; // Terminate the client connection
                }
            }
        }
        buffer.clear();
    }
}

// Define a function to save a file
fn save_file(file_path: &str, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;
    Ok(())
}