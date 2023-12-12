use actix_web::{web, App, HttpServer, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../index.html"))
}

async fn send_message(data: web::Json<MessageData>) -> HttpResponse {
    let message = &data.message;
    // Process the message as needed
    HttpResponse::Ok().json(MessageResponse {
        message: format!("Received message: {}", message),
    })
}

pub async fn actix_web_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/send", web::post().to(send_message))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(serde::Deserialize)]
struct MessageData {
    message: String,
}

#[derive(serde::Serialize)]
struct MessageResponse {
    message: String,
}