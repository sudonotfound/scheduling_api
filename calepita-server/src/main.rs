use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // .env ファイルの読み込み（任意）
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "カレピタへようこそ！"
}
