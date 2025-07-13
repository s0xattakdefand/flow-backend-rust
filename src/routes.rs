use axum::Router;
use crate::handlers::user_handler::register_routes;

pub async fn create_app() -> Router {
    Router::new()
        .nest("/users", register_routes())
}

