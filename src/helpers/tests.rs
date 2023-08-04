use sqlx::PgPool;

use crate::{
    models::user::User,
    repositories::user_repository::{CreateUser, UserRepository, UserRepositoryError},
};

use super::crypto::sha256_hmac;

pub async fn create_test_pool() -> PgPool {
    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    pool
}

pub async fn create_test_user() -> Result<User, UserRepositoryError> {
    let pool = create_test_pool().await;
    let user_repo = UserRepository::new(pool.clone());

    let test_user = CreateUser {
        email: Some(String::from("test@test.com")),
        password: Some(sha256_hmac(
            "password",
            dotenvy::var("APP_KEY").unwrap().as_str(),
        )),
        npub: None,
        identifier: None,
    };

    let user = user_repo.create(&test_user).await?;

    Ok(user)
}

pub async fn delete_test_user(uuid: &str) -> Result<bool, UserRepositoryError> {
    let pool = create_test_pool().await;
    let user_repo = UserRepository::new(pool.clone());

    let result = user_repo.hard_delete(uuid).await?;
    Ok(result)
}
