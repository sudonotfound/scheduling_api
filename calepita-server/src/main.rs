use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing_subscriber;

mod routes;
mod firestore;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(routes::health_check))
        .route("/schedules", get(routes::get_schedules)); // 仮置き

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
