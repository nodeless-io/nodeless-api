use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow, Clone)]
pub struct Checkout {
    pub uuid: String,
    pub user_uuid: String,
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

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "checkout_status", rename_all = "lowercase")]
pub enum CheckoutStatus {
    New,
    PendingConfirmation,
    Paid,
    Overpaid,
    Underpaid,
    Expired,
}

impl Display for CheckoutStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckoutStatus::New => write!(f, "new"),
            CheckoutStatus::PendingConfirmation => write!(f, "pending_confirmation"),
            CheckoutStatus::Paid => write!(f, "paid"),
            CheckoutStatus::Overpaid => write!(f, "overpaid"),
            CheckoutStatus::Underpaid => write!(f, "underpaid"),
            CheckoutStatus::Expired => write!(f, "expired"),
        }
    }
}
