use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct Store {
    pub uuid: String,
    pub user_uuid: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct StoreInvoice {
    pub uuid: String,
    pub store_uuid: String,
    pub checkout_uuid: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
