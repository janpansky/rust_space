use actix_web::{web, App, HttpServer, HttpResponse};
use prometheus::{register_counter, Counter, Encoder, TextEncoder};
use actix_web::middleware::Logger;
use std::sync::Mutex;

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

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

async fn send_message(data: web::Json<MessageData>) -> HttpResponse {
    let message = &data.message;
    MESSAGES_SENT.inc();  // Increment the messages sent counter
    // Process the message as needed
    HttpResponse::Ok().json(MessageResponse {
        message: format!("Received message: {}", message),
    })
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
            .route("/", web::get().to(index))
            .route("/send", web::post().to(send_message))
            .route("/metrics", web::get().to(metrics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}