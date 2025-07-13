use crate::models::user::User;
use sqlx::{query_as, PgPool};
use chrono::Utc;

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, created_at
        "#,
        username,
        email,
        password_hash,
        Utc::now()
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, sqlx::Error> {
    query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at
        FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_one(pool)
    .await
}
