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
    env::set_var("DATABASE_URL", "sqlite://database.sqlite");

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
    let password_hash = "dummy_hash";

    // Connect to the database
    let pool = SqlitePool::connect_lazy_with(options);

    let listener = TcpListener::bind("0.0.0.0:11111").await
        .context("Failed to bind server to address")?;

    accept_connections(listener, pool, password_hash).await?;

    Ok(())
}

async fn accept_connections(
    listener: TcpListener,
    pool: SqlitePool,
    password_hash: &str,
) -> Result<(), Box<dyn Error>> {
    // Continuously accept incoming connections
    while let Ok((socket, addr)) = listener.accept().await {
        // Clone the pool for the asynchronous task
        let pool = pool.clone();

        // Clone password_hash to ensure 'static lifetime
        let password_hash = password_hash.to_string();

        // Spawn an asynchronous task for each incoming connection
        tokio::spawn(async move {
            // Attempt to create a new user
            let user_id = match create_user(&pool, addr, &password_hash).await {
                Ok(id) => id,
                Err(err) => {
                    // Print an error message and return if user creation fails
                    eprintln!("Failed to create user: {:?}", err);
                    return;
                }
            };

            // Handle the client connection for the newly created user
            handle_client(socket, pool, user_id).await;
        });
    }

    // Return Ok(()) when all connections are handled
    Ok(())
}

async fn handle_client(mut socket: TcpStream, pool: SqlitePool, user_id: i64) {
    let client_addr = socket.peer_addr().unwrap();
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
                    save_text_message(&pool, user_id, 1, &text)
                        .await
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

async fn create_user(
    pool: &SqlitePool,
    client_addr: std::net::SocketAddr,
    password_hash: &str,
) -> Result<i64, sqlx::Error> {
    let mut username = client_addr.ip().to_string();

    // Append a timestamp to the username
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    username.push_str(&timestamp);

    let user_id = sqlx::query_scalar(
        "INSERT INTO users (username, password_hash) VALUES (?, ?) RETURNING id",
    )
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

    Ok(user_id)
}

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

// Define a function to save a file
fn save_file(file_path: &str, content: &[u8]) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content)?;
    Ok(())
}