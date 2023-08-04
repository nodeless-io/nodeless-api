use crate::models::user::User;
use sqlx::PgPool;
use sqlx::Row;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository { pool }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, UserRepositoryError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_password(&self, email: &str) -> Result<String, UserRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT password
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        let password: String = row.get(0);
        Ok(password)
    }
}
