use std::error::Error;
use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;

async fn test() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/api/test", get(test).post(test));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
