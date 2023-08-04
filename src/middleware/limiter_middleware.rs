use actix_web::{dev, web, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use futures_util::FutureExt;
use moka::future::Cache;
use std::sync::Arc;

use crate::helpers::crypto::sha256_hmac;

#[derive(Clone)]
pub struct ApiLimiter {
    pub cache: Arc<Cache<String, u32>>,
}

impl FromRequest for ApiLimiter {
    type Error = Error;
    type Future = Ready<Result<ApiLimiter, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            let token = auth_header.to_str().unwrap().to_string();
            let api_limiter = req
                .app_data::<ApiLimiter>()
                .expect("Failed to get ApiLimiter cache from request");

            let count = api_limiter.cache.get(&token);
            async {
                if count.is_some() {
                    api_limiter.cache.insert(token, count.unwrap() + 1).await;
                } else {
                    api_limiter.cache.insert(token, 1).await;
                }
            }
            .boxed_local();

            if count.is_some() && count.unwrap() > 10 {
                return ready(Err(actix_web::error::ErrorTooManyRequests(
                    "Too many requests",
                )));
            }

            ready(Ok(api_limiter.clone()))
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized(
                "Missing api token",
            )))
        }
    }
}

#[derive(Clone)]
pub struct GuestLimiter {
    pub cache: Arc<Cache<String, u32>>,
}

impl FromRequest for GuestLimiter {
    type Error = Error;
    type Future = Ready<Result<GuestLimiter, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        eprintln!("GuestLimiter");
        let user_ip = req.connection_info().peer_addr().unwrap().to_string();
        let hashed_ip = sha256_hmac(user_ip.as_str(), dotenvy::var("APP_KEY").unwrap().as_str());
        let guest_limiter = req
            .app_data::<GuestLimiter>()
            .expect("Failed to get GuestLimiter cache from request");

        let count = guest_limiter.cache.get(&hashed_ip);
        async {
            if count.is_some() {
                guest_limiter
                    .cache
                    .insert(hashed_ip, count.unwrap() + 1)
                    .await;
                eprintln!("count: {}", count.unwrap());
            } else {
                guest_limiter.cache.insert(hashed_ip, 1).await;
                eprintln!("count: {}", 1);
            }
        }
        .boxed_local();

        if count.is_some() && count.unwrap() > 2 {
            return ready(Err(actix_web::error::ErrorTooManyRequests(
                "Too many requests",
            )));
        }
        ready(Ok(guest_limiter.clone()))
    }
}

pub async fn guest_limiter(req: &HttpRequest, cache: web::Data<GuestLimiter>, limit: u32) -> bool {
    let user_ip = req.connection_info().peer_addr().unwrap().to_string();
    let hashed_ip = sha256_hmac(user_ip.as_str(), dotenvy::var("APP_KEY").unwrap().as_str());
    let count = cache.cache.get(&hashed_ip);
    if count.is_some() && count.unwrap() >= limit {
        return false;
    }
    if count.is_none() {
        cache.cache.insert(hashed_ip, 1).await;
    } else {
        cache.cache.insert(hashed_ip, count.unwrap() + 1).await;
    }
    true
}
