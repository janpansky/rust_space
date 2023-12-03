use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;
use chrono::Utc;
use log::{info};
use anyhow::{Context, Result};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePool;

extern crate shared_library;

use shared_library::{MessageType, create_directories};

const BUFFER_SIZE: usize = 16384;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Set the DATABASE_URL before connecting to the database
    std::env::set_var("DATABASE_URL", "sqlite://database.sqlite");

    // Create directories if they don't exist
    create_directories()?;

    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt::init();

    // Construct the correct SQLite database URL
    println!("{:?}", env::current_dir());
    let database_path = env::current_dir().unwrap().join("database.sqlite");

    if !database_path.exists() {
        File::create(&database_path)
            .expect("Failed to create database file");
    }

    let options = SqliteConnectOptions::new().filename(&database_path).create_if_missing(true);

    // Placeholder for user identification
    let username = "test";
    let password_hash = "test_hash";

    // Connect to the database
    let pool = SqlitePool::connect_lazy_with(options);

    match create_user(&pool, username, password_hash).await {
        Ok(sender_id) => {
            let receiver_id = 1/* you need to get a valid receiver_id */;
            let content = "Hello, this is a test message.";

            if let Err(err) = save_text_message(&pool, sender_id, receiver_id, content).await {
                eprintln!("Failed to save text message: {:?}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to create user: {:?}", err);
        }
    }

    // Perform any necessary database migrations here
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Bind the server to the specified address and port
    let listener = TcpListener::bind("0.0.0.0:11111").await
        .context("Failed to bind server to address")?;
    // Use context to add additional information
    info!("Server listening on 0.0.0.0:11111");

    // Accept incoming connections and spawn a new task to handle each one
    while let Ok((socket, _)) = listener.accept().await {
        // Clone the database pool for each client handler
        let pool = pool.clone();
        tokio::spawn(async move {
            handle_client(socket, pool).await;
        });
    }

    Ok(())
}

// Asynchronously handle a connected client
async fn handle_client(mut socket: TcpStream, pool: SqlitePool) {
    // Placeholder for user identification
    let user_id = identify_user(&pool /* any identification data */).await;

    let client_addr = socket.peer_addr().unwrap();
    // Get the client's address
    info!("Client connected from: {}", client_addr);

    // Handle incoming messages from clients - Vector size manually adjusted
    let mut buffer = vec![0u8; BUFFER_SIZE];
    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        };

        if n == 0 {
            break; // Connection closed by the client
        }

        // Deserialize the received message into MessageType enum
        if let Ok(message) = serde_cbor::from_slice::<MessageType>(&buffer[0..n]) {
            match message {
                MessageType::File(filename, file_content) => {
                    // Handle file transfer
                    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
                    let file_path = format!("files/{}.txt", timestamp);
                    save_file(&file_path, &file_content)
                        .with_context(|| format!("Failed to save file: {}", filename))
                        .unwrap();
                    // Unwrap is safe here as we are stopping the loop on error
                    info!(
                        "Received file from {}: {}, saving as {}",
                        client_addr, filename, file_path
                    );
                }
                MessageType::Image(filename, image_content) => {
                    // Handle image transfer
                    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
                    let file_path = format!("images/{}.png", timestamp);
                    save_file(&file_path, &image_content)
                        .with_context(|| format!("Failed to save image: {}", filename))
                        .unwrap();
                    // Unwrap is safe here as we are stopping the loop on error
                    info!(
                        "Received image from {}: {}, saving as {}",
                        client_addr, filename, file_path
                    );
                }
                MessageType::Text(text) => {
                    // Handle text message
                    info!("Received text from {}: {}", client_addr, text);

                    // Save the text message to the database
                    save_text_message(&pool, user_id.unwrap(), 1, &text).await
                        .with_context(|| format!("Failed to save text message: {}", text))
                        .unwrap();
                }
                MessageType::Quit => {
                    info!("Client from {} requested termination.", client_addr);
                    break; // Terminate the client connection
                }
            }
        }

        // Reset the buffer for the next iteration
        buffer.iter_mut().for_each(|b| *b = 0u8);
    }

    // Log that the client connection is closed
    info!("Client from {} connection closed.", client_addr);
}

// Define a function to save a file
fn save_file(file_path: &str, content: &[u8]) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;
    Ok(())
}

// Asynchronously create a user (placeholder implementation)
async fn create_user(pool: &SqlitePool, username: &str, password_hash: &str) -> Result<i64, sqlx::Error> {
    let user_id = sqlx::query_scalar(
        "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id",
    )
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

    Ok(user_id)
}

// Asynchronously save a text message
async fn save_text_message(
    pool: &SqlitePool,
    sender_id: i64,
    receiver_id: i64,
    content: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO chat_messages (sender_id, receiver_id, content) VALUES (?, ?, ?)",
        sender_id,
        receiver_id,
        content
    )
        .execute(pool)
        .await?;

    Ok(())
}

// Asynchronously identify the user (placeholder implementation)
async fn identify_user(pool: &SqlitePool /* any identification data */) -> Option<i64> {
    Some(1)
}