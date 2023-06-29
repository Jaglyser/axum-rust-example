use axum::response::Json;
use serde_json::Value;
use crate::services::example_service::example_service;

pub async fn example_handler() -> Json<Value> {
    example_service()
}
