use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

pub async fn get_schedules() -> impl IntoResponse {
    Json(json!({ "schedules": [] })) // 後で Firestore 結合
}
