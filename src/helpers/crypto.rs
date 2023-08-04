use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub fn sha256_hmac(data: &str, secret: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(data.as_bytes());

    let result = mac.finalize();
    let result_bytes = result.into_bytes();

    hex::encode(result_bytes)
}
