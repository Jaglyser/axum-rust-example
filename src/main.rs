mod routers;
mod handlers;
mod services;

use axum::Router;
use routers::example_router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(example_router::route());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

