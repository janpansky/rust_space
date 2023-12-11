//! # Robot Dreams Actix-Web Server
//!
//! This module contains the Actix-Web server implementation for the Robot Dreams application.
//! The server handles incoming HTTP requests, interacts with a SQLite database to fetch user
//! information and chat messages, and provides API endpoints for clients to retrieve data.
//!
//! ## Modules
//!
//! - [`main`]: Contains the main entry point for the Actix-Web server application.
//! - [`index`]: Defines the handler for the root route ("/") that serves an HTML file.
//! - [`get_messages`]: Defines the handler for the "/messages" route that fetches messages from the database.
//! - [`get_users`]: Defines the handler for the "/users" route that fetches user information from the database.
//! - [`get_chat_messages`]: Defines the handler for the "/chat_messages" route that fetches chat messages from the database.
//! - [`tests`]: Contains unit tests for the server.
//!
//! ## Usage
//!
//! To run the server, execute the binary produced by the compilation process.
//!
//! ```
//! cargo run --bin robotdreams
//! ```

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sqlx::{Sqlite, SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Mutex;

/// Represents a user in the system.
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct User {
    id: i64,
    username: String,
    password_hash: String,
    created_at: String,
}

/// Represents a chat message.
#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct ChatMessage {
    id: i64,
    sender_id: i64,
    receiver_id: i64,
    content: String,
    timestamp: String,
}

/// Main entry point for the Actix-Web server application.
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the database URL
    let database_url = "sqlite://database.sqlite";
    let pool = SqlitePoolOptions::new()
        .connect_lazy_with(database_url.parse().unwrap_or_else(|_| {
            panic!("Failed to parse database URL: {}", database_url);
        }));

    // Wrap the pool in a Mutex to safely share it between threads
    let pool = web::Data::new(Mutex::new(pool));

    // Start Actix-Web server
    HttpServer::new(move || {
        App::new()
            // Data is used to share the SqlitePool between handlers
            .app_data(pool.clone())
            .route("/", web::get().to(index))
            .route("/messages", web::get().to(get_messages))
            .route("/users", web::get().to(get_users))
            .route("/chat_messages", web::get().to(get_chat_messages))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

/// Defines the handler for the root route ("/") that serves an HTML file.
async fn index() -> HttpResponse {
    // Serve your HTML file here
    HttpResponse::Ok().body("Hello, Actix!")
}

/// Defines the handler for the "/messages" route that fetches messages from the database.
async fn get_messages() -> HttpResponse {
    // Fetch and return messages from the database
    HttpResponse::Ok().body("Messages will be fetched from the database.")
}

/// Defines the handler for the "/users" route that fetches user information from the database.
async fn get_users(pool: web::Data<Mutex<SqlitePool>>) -> impl Responder {
    let pool = pool.lock().unwrap();
    let result = sqlx::query_as::<Sqlite, User>("SELECT * FROM users")
        .fetch_all(&*pool)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Defines the handler for the "/chat_messages" route that fetches chat messages from the database.
async fn get_chat_messages(pool: web::Data<Mutex<SqlitePool>>) -> impl Responder {
    let pool = pool.lock().unwrap();
    let result = sqlx::query_as::<Sqlite, ChatMessage>("SELECT * FROM chat_messages")
        .fetch_all(&*pool)
        .await;

    match result {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Error fetching chat messages: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_service::Service;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_get_users() {
        let pool = web::Data::new(Mutex::new(create_test_pool()));

        let req = test::TestRequest::get().uri("/users").app_data(pool.clone()).to_request();
        let resp = test::call_service(&create_test_app(), req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}