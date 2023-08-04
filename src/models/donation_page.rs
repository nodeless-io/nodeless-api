use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct DonationPage {
    pub uuid: String,
    pub user_uuid: String,
    pub slug: String,
    pub name: String,
    pub description: String,
}
