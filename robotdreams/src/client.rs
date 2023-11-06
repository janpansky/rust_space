use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use image::{DynamicImage, ImageFormat}; // Import image-related items

#[derive(Serialize, Deserialize)]
enum MessageType {
    File(String, Vec<u8>), // Filename and its content as bytes
    Image(Vec<u8>), // Image content as bytes
    Text(String),
    Quit,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let server_addr = "0.0.0.0:11111";
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Connected to server at {}", server_addr);

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    let images_dir = "images";
    let files_dir = "files";

    // Create directories if they don't exist
    std::fs::create_dir_all(images_dir)?;
    std::fs::create_dir_all(files_dir)?;

    loop {
        let mut input = String::new();
        println!("Enter a message (or type '.quit' to exit):");
        reader.read_line(&mut input).await?;

        if input.trim() == ".quit" {
            let quit_message = MessageType::Quit;
            let message_bytes = serde_cbor::to_vec(&quit_message)?;
            stream.write_all(&message_bytes).await?;
            break;
        } else {
            if input.starts_with(".file") {
                let filename = input.trim_start_matches(".file ").trim();
                let file_content = Vec::new(); // Read file content from the file
                // You should read the file content here and store it in the `file_content` vector.

                // Save the file to the files directory
                let file_path = format!("{}/{}", files_dir, filename);
                println!("filepath: {}", file_path);
                let mut file = File::create(&file_path)?;
                file.write_all(&file_content)?;

                println!("Receiving file: {}", filename);

                let file_message = MessageType::File(filename.to_string(), file_content);
                let message_bytes = serde_cbor::to_vec(&file_message)?;
                stream.write_all(&message_bytes).await?;
            } else if input.starts_with(".image") {
                let image_content = Vec::new(); // Read image content from the input
                // You should read the image content here and store it in the `image_content` vector.

                // Convert the received image content to a DynamicImage
                let dynamic_image = image::load_from_memory(&image_content)?;

                // Generate a timestamp for the filename
                let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
                let filename = format!("{}.png", timestamp);

               // Save the image to the images directory as PNG
                let image_path = format!("{}/{}", images_dir, filename);
                // Save the image in PNG format
                dynamic_image.save_with_format(image_path, ImageFormat::Png)?;

                println!("Receiving image: {}", filename);

                let image_message = MessageType::Image(image_content);
                let message_bytes = serde_cbor::to_vec(&image_message)?;
                stream.write_all(&message_bytes).await?;
            } else {
                let text_message = MessageType::Text(input.clone());
                let message_bytes = serde_cbor::to_vec(&text_message)?;
                stream.write_all(&message_bytes).await?;
            }
        }
    }

    Ok(())
}