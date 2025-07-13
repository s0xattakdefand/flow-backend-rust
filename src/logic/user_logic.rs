use crate::{db::user_repo, models::user::User, errors::ApiError};
use argon2::{Argon2, PasswordHasher};
use password_hash::{SaltString, rand_core::OsRng};

pub async fn create_user_logic(
    pool: &sqlx::PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, ApiError> {
    let password_hash = hash_password(password)?;
    match user_repo::create_user(pool, username, email, &password_hash).await {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(err)) if err.message().contains("duplicate") => {
            Err(ApiError::InvalidInput("Username or email already exists".into()))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ApiError::InternalServerError)?
        .to_string();

    Ok(hash)
}
