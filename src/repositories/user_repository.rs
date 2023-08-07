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

pub struct CreateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub npub: Option<String>,
    pub identifier: Option<String>,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository { pool }
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, UserRepositoryError> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (uuid, email, password, npub, identifier)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(uuid)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.npub)
        .bind(&user.identifier)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
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

    pub async fn delete(&self, uuid: &str) -> Result<(), UserRepositoryError> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE uuid = $1
            "#,
        )
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn hard_delete(&self, uuid: &str) -> Result<bool, UserRepositoryError> {
        let rows = sqlx::query(
            r#"
            DELETE FROM users
            WHERE uuid = $1
            "#,
        )
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(rows.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {}
