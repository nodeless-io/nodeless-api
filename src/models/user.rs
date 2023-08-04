use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub password: String,
    pub npub: Option<String>,
    pub identifier: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
