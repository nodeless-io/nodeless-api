use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::store::{Store, StoreInvoice};

#[derive(Debug, Clone)]
pub struct StoreRepository {
    pool: PgPool,
}

impl StoreRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, user_uuid: &str, name: &str) -> Result<Store, Error> {
        let uuid = Uuid::new_v4().to_string();
        let now = chrono::Local::now().naive_utc();

        let store = sqlx::query_as::<_, Store>(
            "INSERT INTO stores (uuid, user_uuid, name, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(&uuid)
        .bind(user_uuid)
        .bind(name)
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(store)
    }

    pub async fn get_all(&self, user_uuid: &str) -> Result<Vec<Store>, Error> {
        let stores = sqlx::query_as::<_, Store>(
            "SELECT * FROM stores WHERE user_uuid = $1 AND deleted_at IS NULL",
        )
        .bind(user_uuid)
        .fetch_all(&self.pool)
        .await?;

        Ok(stores)
    }

    pub async fn get_by_uuid(
        &self,
        user_uuid: &str,
        store_uuid: &str,
    ) -> Result<Option<Store>, Error> {
        let store = sqlx::query_as::<_, Store>(
            "SELECT * FROM stores WHERE uuid = $1 AND user_uuid = $2 AND deleted_at IS NULL",
        )
        .bind(store_uuid)
        .bind(user_uuid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(store)
    }

    pub async fn update(
        &self,
        user_uuid: &str,
        store_uuid: &str,
        name: &str,
    ) -> Result<Option<Store>, Error> {
        let now = chrono::Local::now().naive_utc();

        let store = sqlx::query_as::<_, Store>(
            "UPDATE stores SET name = $1, updated_at = $2 WHERE uuid = $3 AND user_uuid = $4 AND deleted_at IS NULL RETURNING *"
        )
        .bind(name)
        .bind(&now)
        .bind(store_uuid)
        .bind(user_uuid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(store)
    }

    pub async fn delete(&self, user_uuid: &str, store_uuid: &str) -> Result<bool, Error> {
        let now = chrono::Local::now().naive_utc();

        let store = sqlx::query(
            "UPDATE stores SET deleted_at = $1 WHERE uuid = $2 AND user_uuid = $3 AND deleted_at IS NULL"
        )
        .bind(&now)
        .bind(store_uuid)
        .bind(user_uuid)
        .execute(&self.pool)
        .await?;

        Ok(store.rows_affected() > 0)
    }

    pub async fn hard_delete(&self, user_uuid: &str, store_uuid: &str) -> Result<bool, Error> {
        let store = sqlx::query("DELETE FROM stores WHERE uuid = $1 AND user_uuid = $2")
            .bind(store_uuid)
            .bind(user_uuid)
            .execute(&self.pool)
            .await?;

        Ok(store.rows_affected() > 0)
    }
}

pub struct StoreInvoiceRepository {
    pool: PgPool,
}

pub struct CreateStoreInvoice {
    pub store_uuid: String,
    pub checkout_uuid: String,
    pub metadata: Option<serde_json::Value>,
}

impl StoreInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, invoice: CreateStoreInvoice) -> Result<StoreInvoice, Error> {
        let uuid = Uuid::new_v4().to_string();
        let invoice = sqlx::query_as::<_, StoreInvoice>(
            "INSERT INTO store_invoices 
            (uuid, store_uuid, checkout_uuid, metadata)
            VALUES 
            ($1, $2, $3, $4) RETURNING *",
        )
        .bind(&uuid)
        .bind(invoice.store_uuid)
        .bind(invoice.checkout_uuid)
        .bind(invoice.metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(invoice)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{create_test_pool, create_test_user, delete_test_user};

    use super::StoreRepository;

    #[tokio::test]
    async fn test_store_crud() {
        let user = create_test_user().await.unwrap();
        let pool = create_test_pool().await;
        let store_repo = StoreRepository::new(pool);
        let store = store_repo.create(&user.uuid, "test store").await.unwrap();

        assert_eq!(store.user_uuid, user.uuid);
        assert_eq!(store.name, "test store");

        let store = store_repo
            .get_by_uuid(&user.uuid, &store.uuid)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(store.user_uuid, user.uuid);
        assert_eq!(store.name, "test store");

        let store = store_repo
            .update(&user.uuid, &store.uuid, "test store 2")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(store.user_uuid, user.uuid);
        assert_eq!(store.name, "test store 2");

        let soft_delete = store_repo.delete(&user.uuid, &store.uuid).await.unwrap();

        assert_eq!(soft_delete, true);

        let delete = store_repo
            .hard_delete(&user.uuid, &store.uuid)
            .await
            .unwrap();

        assert_eq!(delete, true);

        let deleted_user = delete_test_user(&user.uuid).await.unwrap();

        assert_eq!(deleted_user, true);
    }
}
