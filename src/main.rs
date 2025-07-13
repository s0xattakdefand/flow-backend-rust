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

use db::init_db_pool;
use routes::create_app;
use dotenv::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Load environment variables from `.env`
    dotenv().ok();

    // Initialize DB pool
    let pool = db::init_db_pool().await;

    // Create the Axum app with routes and state
    let app = routes::create_app(pool).await;

    // Bind and serve
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("🚀 Server running at http://127.0.0.1:8080");

    axum::serve(listener, app).await.unwrap();
}