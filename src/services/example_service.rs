use axum::response::Json;
use serde_json::{Value, json};


pub fn example_service() -> Json<Value> {
    Json(json!({ "message": "Hello world" }))
}

