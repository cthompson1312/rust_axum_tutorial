use axum::{routing::get, routing::post, Router};
#[path = "./endpoints/bingo.rs"]
mod bingo;
#[path = "./endpoints/index.rs"]
mod index;
#[path = "./endpoints/weather.rs"]
mod weather;

#[tokio::main]
async fn main() {
    const PORT: &str = "8787";

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .expect("Failed to bind TCP listener.");

    let router = Router::new()
        .route("/", get(index::index_handler))
        .route("/weather", get(weather::weather_handler))
        .route("/bingo", post(bingo::bingo_handler));

    axum::serve(listener, router)
        .await
        .expect("Failed to start server.");
}
