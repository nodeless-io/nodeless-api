use sqlx::PgPool;
use uuid::Uuid;

use crate::models::nodeless_address::NodelessAddress;

pub struct NodelessAddressRepository {
    pub pool: PgPool,
}

pub struct CreateNodelessAddress {
    pub user_uuid: String,
    pub handle: String,
    pub npub: Option<String>,
    pub price: i64,
}

pub struct UpdateNodelessAddress {
    pub npub: Option<String>,
    pub price: Option<i64>,
}

impl NodelessAddressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, addr: CreateNodelessAddress) -> Result<NodelessAddress, sqlx::Error>  {
        let uuid = Uuid::new_v4().to_string();

        let addr = sqlx::query_as::<_, NodelessAddress>(
            r#"
            INSERT INTO nodeless_addresses (uuid, user_uuid, handle, npub, price)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(uuid)
        .bind(addr.user_uuid)
        .bind(addr.handle)
        .bind(addr.npub)
        .bind(addr.price as i64)
        .fetch_one(&self.pool)
        .await?;

        Ok(addr)
    }

    pub async fn get_by_handle(&self, handle: &str) -> Result<NodelessAddress, sqlx::Error> {
        let addr = sqlx::query_as::<_, NodelessAddress>(
            r#"
            SELECT * FROM nodeless_addresses
            WHERE handle = $1
            "#,
        )
        .bind(handle)
        .fetch_one(&self.pool)
        .await?;

        Ok(addr)
    }

    pub async fn get_by_uuid(&self, uuid: &str) -> Result<NodelessAddress, sqlx::Error> {
        let addr = sqlx::query_as::<_, NodelessAddress>(
            r#"
            SELECT * FROM nodeless_addresses
            WHERE uuid = $1
            "#,
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(addr)
    }

    pub async fn update(&self, uuid: &str, data: UpdateNodelessAddress) -> Result<NodelessAddress, sqlx::Error> {
        let addr = sqlx::query_as::<_, NodelessAddress>(
            r#"
            UPDATE nodeless_addresses
            SET npub = $1, price = $2
            WHERE uuid = $3
            RETURNING *
            "#,
        )
        .bind(data.npub)
        .bind(data.price)
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(addr)
    }

    pub async fn delete(&self, uuid: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE nodeless_addresses SET deleted_at = NOW()
            WHERE uuid = $1
            "#,
        ).bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn hard_delete(&self, uuid: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM nodeless_addresses
            WHERE uuid = $1
            "#,
        ).bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::helpers::tests::{create_test_pool, create_test_user, delete_test_user};

    #[tokio::test]
    async fn test_nodeless_address_crud() {
        let pool = create_test_pool().await;
        let user = create_test_user().await.unwrap();
        let user_clone = user.clone();

        let repo = super::NodelessAddressRepository::new(pool.clone());

        let addr = repo.create(super::CreateNodelessAddress {
            user_uuid: user.uuid,
            handle: "test".to_string(),
            npub: None,
            price: 0,
        }).await.unwrap();

        assert_eq!(&addr.user_uuid, &user_clone.uuid);
        assert_eq!(addr.handle, "test");
        assert_eq!(addr.npub, None);
        assert_eq!(addr.price, 0);

        let addr = repo.get_by_handle("test").await.unwrap();

        assert_eq!(&addr.user_uuid, &user_clone.uuid);
        assert_eq!(addr.handle, "test");
        assert_eq!(addr.npub, None);
        assert_eq!(addr.price, 0);

        let addr = repo.get_by_uuid(&addr.uuid).await.unwrap();

        assert_eq!(&addr.user_uuid, &user_clone.uuid);
        assert_eq!(addr.handle, "test");
        assert_eq!(addr.npub, None);
        assert_eq!(addr.price, 0);

        let addr = repo.update(&addr.uuid, super::UpdateNodelessAddress {
            npub: Some("test".to_string()),
            price: Some(100),
        }).await.unwrap();

        assert_eq!(&addr.user_uuid, &user_clone.uuid);
        assert_eq!(addr.handle, "test");
        assert_eq!(addr.npub, Some("test".to_string()));
        assert_eq!(addr.price, 100);

        let delete = repo.delete(&addr.uuid).await.unwrap();

        assert_eq!(delete, true);

        let hard_delete = repo.hard_delete(&addr.uuid).await.unwrap();

        assert_eq!(hard_delete, true);

        let _ = delete_test_user(&user_clone.uuid).await.unwrap();
    }
}