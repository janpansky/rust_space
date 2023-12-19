use actix_web::{web, App, HttpServer, HttpResponse};
use prometheus::{register_counter, Counter, Encoder, TextEncoder};
use actix_web::middleware::Logger;
use actix_files::Files;
use std::sync::Mutex;
use std::str::FromStr;

extern crate shared_library;

use shared_library::{MessageType};

#[derive(serde::Deserialize)]
struct MessageData {
    message: String,
}

#[derive(serde::Serialize)]
struct MessageResponse {
    message: String,
}

lazy_static::lazy_static! {
    static ref MESSAGES_SENT: Counter = register_counter!(
        "messages_sent_total",
        "Total number of messages sent"
    ).unwrap();
}

// Define a wrapper type around MessageType
pub struct MessageTypeWrapper(pub MessageType);

impl Default for MessageTypeWrapper {
    fn default() -> Self {
        MessageTypeWrapper(MessageType::Text("".to_string()))
    }
}

// Implement FromStr for MessageTypeWrapper
impl FromStr for MessageTypeWrapper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Text" => Ok(MessageTypeWrapper(MessageType::Text("".to_string()))),
            "File" => Ok(MessageTypeWrapper(MessageType::File("".to_string(), Vec::new()))),
            "Image" => Ok(MessageTypeWrapper(MessageType::Image("".to_string(), Vec::new()))),
            "Quit" => Ok(MessageTypeWrapper(MessageType::Quit)),
            _ => {
                log::warn!("Unknown MessageType: {}", s);
                Ok(Default::default()) // Return a default value for unknown types
            }
        }
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

async fn send_message(data: web::Json<MessageData>, messages_sent: web::Data<Mutex<Counter>>) -> HttpResponse {
    let message_data = &data.message;

    // Acquire lock before incrementing
    let guard = match messages_sent.lock() {
        Ok(guard) => guard,
        Err(e) => {
            log::error!("Failed to acquire lock: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    guard.inc();  // Increment the messages sent counter

    // Log the current count
    log::info!("Messages Sent Count: {}", guard.get());

    // Process the message based on the MessageType enum from the client module
    match MessageTypeWrapper::from_str(message_data) {
        Ok(MessageTypeWrapper(MessageType::Text(text))) => {
            // Handle text message
            // You can replace this with your actual text message handling logic
            println!("Received message_data: {}", message_data);
            HttpResponse::Ok().json(MessageResponse {
                message: format!("Received text message: {}", message_data),
            })
        }
        Ok(MessageTypeWrapper(MessageType::File(filename, content))) => {
            // Handle file message
            // You can replace this with your actual file message handling logic
            HttpResponse::Ok().json(MessageResponse {
                message: format!("Received file message: {} with content length: {}", filename, content.len()),
            })
        }
        Ok(MessageTypeWrapper(MessageType::Image(filename, content))) => {
            // Handle image message
            // You can replace this with your actual image message handling logic
            HttpResponse::Ok().json(MessageResponse {
                message: format!("Received image message: {} with content length: {}", filename, content.len()),
            })
        }
        Ok(MessageTypeWrapper(MessageType::Quit)) => {
            // Handle quit message
            HttpResponse::Ok().json(MessageResponse {
                message: "Received quit message. Closing connection.".to_string(),
            })
        }
        Err(err) => {
            // Handle unknown message types
            log::error!("Error parsing MessageType: {}", err);
            HttpResponse::BadRequest().json(MessageResponse {
                message: "Error parsing MessageType.".to_string(),
            })
        }
        _ => {
            // Handle all other cases with a wildcard pattern
            HttpResponse::BadRequest().json(MessageResponse {
                message: "Unknown MessageType.".to_string(),
            })
        }
    }
}

async fn metrics() -> HttpResponse {
    // Retrieve Prometheus metrics in the Prometheus exposition format
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable logger middleware
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Use a Mutex to make Counter thread-safe
    let messages_sent = web::Data::new(Mutex::new(MESSAGES_SENT.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(messages_sent.clone())
            .wrap(Logger::default())
            .service(Files::new("/static", "./client/static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/send", web::post().to(send_message))
            .route("/metrics", web::get().to(metrics))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}