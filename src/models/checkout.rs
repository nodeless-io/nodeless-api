use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Checkout {
    pub uuid: String,
    pub user_uuid: String,
    pub parent_uuid: String,
    pub parent_type: String,
    pub amount: i64,
    pub status: CheckoutStatus,
    pub bitcoin_address: String,
    pub payment_request: String,
    pub expiry_seconds: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub expired_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "checkout_status", rename_all = "lowercase")]
pub enum CheckoutStatus {
    New,
    PendingConfirmation,
    Paid,
    Overpaid,
    Underpaid,
    Expired,
}
