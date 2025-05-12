use google_cloud_firestore::{Client, Config};
use std::env;

pub async fn init_firestore_client() -> Client {
    let project_id = env::var("PROJECT_ID").expect("PROJECT_ID not set");

    let config = Config {
        project_id,
        ..Default::default()
    };

    Client::new(config).await.expect("Failed to init Firestore client")
}
