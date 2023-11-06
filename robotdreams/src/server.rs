use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::io::{AsyncReadExt};
use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize)]
enum MessageType {
    File(String),
    Image(String),
    Text(String),
    Quit,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error> > {
    let listener = TcpListener::bind("0.0.0.0:11111").await?;
    println!("Server listening on 0.0.0.0:11111");

    while let Ok((socket, _)) = listener.accept().await {
        tokio::spawn(handle_client(socket));
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream) {
    // Handle incoming messages from clients here
    let mut buffer = vec![0u8; 1024];
    while let Ok(n) = socket.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        if let Ok(message) = serde_cbor::from_slice::<MessageType>(&buffer[0..n]) {
            match message {
                MessageType::File(_path) => {
                    // Handle file transfer
                }
                MessageType::Image(_path) => {
                    // Handle image transfer
                }
                MessageType::Text(_text) => {
                    // Handle text message
                }
                MessageType::Quit => {
                    // Handle client quitting
                }
            }
        }
        buffer.clear();
    }
}
