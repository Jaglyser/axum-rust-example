use axum::{Router, routing::get};
use crate::handlers::example_handler::example_handler;

pub fn route() -> Router<()> {
    Router::new().route("/", get(example_handler))
} 
