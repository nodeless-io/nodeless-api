use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub auth: AuthConfig,
    pub pricing: PricingConfig,
    pub stores: StoresConfig,
    pub donation_pages: DonationPagesConfig,
    pub rate_limiter: RateLimiterConfig,
}

impl From<toml::Value> for AppConfig {
    fn from(value: toml::Value) -> Self {
        let auth = value.get("auth").unwrap();
        let pricing = value.get("pricing").unwrap();
        let stores = value.get("stores").unwrap();
        let donation_pages = value.get("donation_pages").unwrap();
        let rate_limiter = value.get("rate_limiter").unwrap();

        AppConfig {
            auth: AuthConfig {
                min_password_length: auth
                    .get("min_password_length")
                    .unwrap()
                    .as_integer()
                    .unwrap() as usize,
                require_strong_password: auth
                    .get("require_strong_password")
                    .unwrap()
                    .as_bool()
                    .unwrap(),
                enable_email_auth: auth.get("enable_email_auth").unwrap().as_bool().unwrap(),
                enable_nost_auth: auth.get("enable_nost_auth").unwrap().as_bool().unwrap(),
                enable_identifier_auth: auth
                    .get("enable_identifier_auth")
                    .unwrap()
                    .as_bool()
                    .unwrap(),
                jwt_expiry_seconds: auth
                    .get("jwt_expiry_seconds")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u64,
            },
            pricing: PricingConfig {
                base_fee_sat: pricing.get("base_fee_sat").unwrap().as_integer().unwrap() as u32,
                fee_rate_percent: pricing
                    .get("fee_rate_percent")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
            },
            stores: StoresConfig {
                max_stores_per_user: stores
                    .get("max_stores_per_user")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
            },
            donation_pages: DonationPagesConfig {
                max_donation_pages_per_user: donation_pages
                    .get("max_donation_pages_per_user")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
            },
            rate_limiter: RateLimiterConfig {
                api_requests_per_second: rate_limiter
                    .get("api_requests_per_second")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
                api_requests_per_minute: rate_limiter
                    .get("api_requests_per_minute")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
                checkout_requests_per_minute: rate_limiter
                    .get("checkout_requests_per_minute")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
                auth_requests_per_hour: rate_limiter
                    .get("auth_requests_per_hour")
                    .unwrap()
                    .as_integer()
                    .unwrap() as u32,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthConfig {
    pub min_password_length: usize,
    pub require_strong_password: bool,
    pub enable_email_auth: bool,
    pub enable_nost_auth: bool,
    pub enable_identifier_auth: bool,
    pub jwt_expiry_seconds: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PricingConfig {
    pub base_fee_sat: u32,
    pub fee_rate_percent: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoresConfig {
    pub max_stores_per_user: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DonationPagesConfig {
    pub max_donation_pages_per_user: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RateLimiterConfig {
    pub api_requests_per_second: u32,
    pub api_requests_per_minute: u32,
    pub checkout_requests_per_minute: u32,
    pub auth_requests_per_hour: u32,
}
