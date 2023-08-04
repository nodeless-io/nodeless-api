use sqlx::PgPool;

use crate::models::checkout::{Checkout, CheckoutStatus};

pub struct CheckoutRepository {
    pub pool: PgPool,
}

pub struct CreateCheckout {
    pub user_uuid: String,
    pub parent_uuid: String,
    pub parent_type: String,
    pub amount: i64,
    pub bitcoin_address: String,
    pub payment_request: String,
    pub expiry_seconds: i64,
}

impl CheckoutRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, req: CreateCheckout) -> Result<Checkout, sqlx::Error> {
        let uuid = uuid::Uuid::new_v4().to_string();
        let checkout = sqlx::query_as::<_, Checkout>(
            r#"
            INSERT INTO checkouts (uuid, user_uuid, parent_uuid, parent_type, amount, status, bitcoin_address, payment_request, expiry_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(uuid)
        .bind(req.user_uuid)
        .bind(req.parent_uuid)
        .bind(req.parent_type)
        .bind(req.amount)
        .bind(CheckoutStatus::New)
        .bind(req.bitcoin_address)
        .bind(req.payment_request)
        .bind(req.expiry_seconds)
        .fetch_one(&self.pool)
        .await?;

        Ok(checkout)
    }

    pub async fn get_by_uuid(&self, uuid: String) -> Result<Checkout, sqlx::Error> {
        let checkout = sqlx::query_as::<_, Checkout>(
            r#"
            SELECT * FROM checkouts WHERE uuid = $1
            "#,
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(checkout)
    }

    pub async fn set_status(&self, uuid: &str, status: CheckoutStatus) -> Result<Checkout, sqlx::Error> {
        let checkout = sqlx::query_as::<_, Checkout>(
            r#"
            UPDATE checkouts SET status = $1 WHERE uuid = $2
            "#,
        )
        .bind(status)
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(checkout)
    }
}

#[cfg(test)]
mod tests {
    use crate::{helpers::tests::{create_test_pool, create_test_user, delete_test_user}, repositories::store_repository::StoreRepository};

    use super::{CheckoutRepository, CreateCheckout};

    #[tokio::test]
    async fn test_create_checkout() {
        let pool = create_test_pool().await;
        let user = create_test_user().await.unwrap();
        let store_repo = StoreRepository::new(pool.clone());
        let store = store_repo.create(&user.uuid, "test store").await.unwrap();
        let checkout_repo = CheckoutRepository::new(pool.clone());
        let user_clone = user.clone();
        let store_clone = store.clone();
        let checkout = CreateCheckout {
            user_uuid: user.uuid,
            parent_uuid: store.uuid,
            parent_type: "store".to_string(),
            amount: 100,
            bitcoin_address: "test address".to_string(),
            payment_request: "test payment request".to_string(),
            expiry_seconds: 100,
        };
        
        let checkout = checkout_repo.create(checkout).await.unwrap();
        assert_eq!(checkout.user_uuid, user_clone.uuid);

        let _ = store_repo.hard_delete(&user_clone.uuid, &store_clone.uuid).await.unwrap();

        sqlx::query!(
            r#"
            DELETE FROM checkouts WHERE uuid = $1
            "#,
            checkout.uuid
        )
        .execute(&pool).await.unwrap();

        let _ = delete_test_user(&user_clone.uuid).await.unwrap();
        
    }
}


