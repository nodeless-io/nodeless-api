use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}