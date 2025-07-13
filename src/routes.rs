use axum::{Router, routing::post};
use crate::handlers::create_user_handler;

pub async fn create_app(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/users", post(create_user_handler))
        .with_state(pool)
}
