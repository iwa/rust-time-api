use axum::{
    routing::get,
    Router,
};
use chrono;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_time))
        .route("/time", get(get_time))

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running");
    axum::serve(listener, app).await.unwrap();
    println!("Server stopped");
}

async fn get_time() -> String {
    let now = chrono::Utc::now();

    now.to_string()
}
