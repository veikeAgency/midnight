use axum::{Router, routing::get, Json, response::IntoResponse};
use serde_json::{json};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health_ping));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("API Listening on http://localhost:3000");

    ax_serve(listener, app).await.unwrap();
}

async fn home() -> impl IntoResponse {
    let _ = Json(json!({
        "message": "Welcome to Midnight"
    }));
}


async fn health_ping() -> impl IntoResponse {
    let _ = Json(json!({
        "status": "ok",
        "server": "midnight",
        "version": "0.0.1"
    }));
}

async fn ax_serve(listener: TcpListener, app: Router) -> std::io::Result<()>{
    axum::serve(listener, app).await
}