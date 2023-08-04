use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn random_text(n: i32) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n as usize)
        .map(char::from)
        .collect()
}