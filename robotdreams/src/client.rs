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
    let server_addr = "localhost:11111";
    let mut stream = TcpStream::connect(server_addr).await?;
    println!("Connected to server at {}", server_addr);

    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);

    let mut line = String::new();

    while let Ok(n) = reader.read_line(&mut line).await {
        if n == 0 {
            break;
        }

        let message = if line.starts_with(".file") {
            let path = line.trim_start_matches(".file ").trim();
            MessageType::File(path.to_string())
        } else if line.starts_with(".image") {
            let path = line.trim_start_matches(".image ").trim();
            MessageType::Image(path.to_string())
        } else if line.trim() == ".quit" {
            MessageType::Quit
        } else {
            MessageType::Text(line.clone())
        };

        let message_bytes = serde_cbor::to_vec(&message)?;
        stream.write_all(&message_bytes).await?;

        line.clear();
    }

    Ok(())
}
