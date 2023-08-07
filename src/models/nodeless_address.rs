use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct NodelessAddress {
    pub uuid: String,
    pub user_uuid: String,
    pub handle: String,
    pub npub: Option<String>,
    pub price: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
