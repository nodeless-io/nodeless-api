use sqlx::PgPool;
use uuid::Uuid;

use crate::models::donation_page::DonationPage;

#[derive(Clone)]
pub struct DonationPageRepository {
    pub pool: PgPool,
}

pub struct CreateDonationPage {
    pub user_uuid: String,
    pub name: String,
    pub slug: String,
    pub description: String,
}

pub struct UpdateDonationPage {
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug)]
pub enum RepoError {
    SlugTaken,
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for RepoError {
    fn from(err: sqlx::Error) -> Self {
        RepoError::DbError(err)
    }
}

impl DonationPageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, page: CreateDonationPage) -> Result<DonationPage, RepoError> {
        let uuid = Uuid::new_v4().to_string();

        if self.slug_exists(&page.slug).await.unwrap() {
            return Err(RepoError::SlugTaken);
        }

        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            INSERT INTO donation_pages (uuid, user_uuid, name, slug, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING uuid, user_uuid, name, slug, description
            "#,
        )
        .bind(uuid)
        .bind(page.user_uuid)
        .bind(page.name)
        .bind(page.slug)
        .bind(page.description)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_one(&self, uuid: &str) -> Result<DonationPage, sqlx::Error> {
        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            SELECT uuid, user_uuid, name, slug, description
            FROM donation_pages
            WHERE uuid = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_one_by_slug(&self, slug: &str) -> Result<DonationPage, sqlx::Error> {
        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            SELECT uuid, user_uuid, name, slug, description
            FROM donation_pages
            WHERE slug = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all_by_user_uuid(
        &self,
        user_uuid: &str,
    ) -> Result<Vec<DonationPage>, sqlx::Error> {
        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            SELECT uuid, user_uuid, name, slug, description
            FROM donation_pages
            WHERE user_uuid = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_uuid)
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_one_by_user_uuid(
        &self,
        user_uuid: &str,
        uuid: &str,
    ) -> Result<DonationPage, sqlx::Error> {
        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            SELECT uuid, user_uuid, name, slug, description
            FROM donation_pages
            WHERE user_uuid = $1 AND uuid = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(user_uuid)
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update(
        &self,
        uuid: &str,
        data: UpdateDonationPage,
    ) -> Result<DonationPage, sqlx::Error> {
        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            UPDATE donation_pages
            SET name = $1, slug = $2, description = $3
            WHERE uuid = $4 AND deleted_at IS NULL
            RETURNING uuid, user_uuid, name, slug, description
            "#,
        )
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn update_by_user_uuid(
        &self,
        uuid: &str,
        user_uuid: &str,
        data: UpdateDonationPage,
    ) -> Result<DonationPage, RepoError> {
        if self.slug_exists(&data.slug).await.unwrap() {
            return Err(RepoError::SlugTaken);
        }

        let result = sqlx::query_as::<_, DonationPage>(
            r#"
            UPDATE donation_pages
            SET name = $1, slug = $2, description = $3
            WHERE uuid = $4 AND user_uuid = $5 AND deleted_at IS NULL
            RETURNING uuid, user_uuid, name, slug, description
            "#,
        )
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(uuid)
        .bind(user_uuid)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn delete(&self, uuid: &str) -> Result<bool, sqlx::Error> {
        let query = sqlx::query!(
            r#"
            UPDATE donation_pages SET deleted_at = NOW() WHERE uuid = $1
            "#,
            uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(query.rows_affected() > 0)
    }

    pub async fn delete_by_user(&self, uuid: &str, user_uuid: &str) -> Result<bool, sqlx::Error> {
        let query = sqlx::query!(
            r#"
            UPDATE donation_pages SET deleted_at = NOW() WHERE uuid = $1 AND user_uuid = $2
            "#,
            uuid,
            user_uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(query.rows_affected() > 0)
    }

    pub async fn hard_delete(&self, uuid: &str) -> Result<bool, sqlx::Error> {
        let query = sqlx::query!(
            r#"
            DELETE FROM donation_pages WHERE uuid = $1
            "#,
            uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(query.rows_affected() > 0)
    }

    async fn slug_exists(&self, slug: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM donation_pages WHERE slug = $1)
            "#,
            slug
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{create_test_pool, create_test_user, delete_test_user};

    #[tokio::test]
    async fn test_donation_page_crud() {
        let pool = create_test_pool().await;
        let user = create_test_user().await.unwrap();
        let user_clone = user.clone();

        let repo = super::DonationPageRepository::new(pool.clone());
        let page = repo
            .create(super::CreateDonationPage {
                user_uuid: user.uuid,
                name: "Test Page".to_string(),
                slug: "test-page".to_string(),
                description: "Test Page Description".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(page.name, "Test Page");
        assert_eq!(page.slug, "test-page");
        assert_eq!(page.description, "Test Page Description");
        assert!(page.uuid.len() > 0);

        let page = repo.get_one(&page.uuid).await.unwrap();

        assert_eq!(page.name, "Test Page");
        assert_eq!(page.slug, "test-page");
        assert_eq!(page.description, "Test Page Description");
        assert!(page.uuid.len() > 0);

        let page = repo.get_one_by_slug("test-page").await.unwrap();

        assert_eq!(page.name, "Test Page");
        assert_eq!(page.slug, "test-page");
        assert_eq!(page.description, "Test Page Description");
        assert!(page.uuid.len() > 0);

        let page = repo
            .update(
                &page.uuid,
                super::UpdateDonationPage {
                    name: "Test Page 2".to_string(),
                    slug: "test-page-2".to_string(),
                    description: "Test Page Description 2".to_string(),
                },
            )
            .await
            .unwrap();

        assert_eq!(page.name, "Test Page 2");
        assert_eq!(page.slug, "test-page-2");
        assert_eq!(page.description, "Test Page Description 2");
        assert!(page.uuid.len() > 0);

        let soft_delete = repo.delete(&page.uuid).await.unwrap();

        assert!(soft_delete);

        let hard_delete = repo.hard_delete(&page.uuid).await.unwrap();

        assert!(hard_delete);

        let _ = delete_test_user(&user_clone.uuid).await.unwrap();
    }
}
