use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncBufReadExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum MessageType {
    File(String),
    Image(String),
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
            let text_message = MessageType::Text(input.clone());
            let message_bytes = serde_cbor::to_vec(&text_message)?;
            stream.write_all(&message_bytes).await?;
        }
    }

    Ok(())
}