use std::error::Error;
use std::net::SocketAddr;

use axum::Json;
use axum::Router;
use axum::routing::get;
use serde::Serialize;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

#[derive(Serialize)]
struct CustomMessage {
    message: String,
}

async fn test() -> Json<CustomMessage> {
    let msg = CustomMessage {
        message: "Hello, World!".to_string(),
    };
    Json(msg)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .fallback_service(
            ServeDir::new("./site").not_found_service(ServeFile::new("./site/index.html")),
        )
        .route("/api/test", get(test).post(test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
