mod routes;
mod errors;
mod handlers;
mod middleware;
mod db;
mod models;
mod logic;
mod validation;
mod transformations;
mod caching;
mod auth;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = routes::create_app().await;

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

