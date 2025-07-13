use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::logic::user_logic;
use crate::errors::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user = user_logic::create_user_logic(
        &pool,
        &payload.username,
        &payload.email,
        &payload.password,
    ).await?;

    let response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
