use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use handlers::frontend::*;
use middleware::limiter_middleware::{ApiLimiter, GuestLimiter};
use moka::future::Cache;
use repositories::{store_repository::StoreRepository, user_repository::UserRepository};
use sqlx::PgPool;
use std::{fs::read_to_string, sync::Arc, time::Duration};
use toml::Value;

pub mod config;
pub mod handlers;
pub mod helpers;
pub mod middleware;
pub mod models;
pub mod repositories;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let user_repo = UserRepository::new(pool.clone());
    let store_repo = StoreRepository::new(pool.clone());
    let config_content = read_to_string("Nodeless.toml").expect("Failed to read Nodeless.toml");
    let toml_config: Value = config_content
        .parse()
        .expect("Failed to parse Nodeless.toml");

    let app_config = config::AppConfig::from(toml_config);

    let api_limiter_cache: Cache<String, u32> = Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .build();
    let api_limiter_cache = Arc::new(api_limiter_cache);

    let api_limiter = ApiLimiter {
        cache: api_limiter_cache,
    };

    let guest_limiter_cache: Cache<String, u32> = Cache::builder()
        .time_to_live(Duration::from_secs(3600))
        .build();
    let guest_limiter_cache = Arc::new(guest_limiter_cache);

    let guest_limiter = GuestLimiter {
        cache: guest_limiter_cache,
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(store_repo.clone()))
            .app_data(Data::new(app_config.clone()))
            .app_data(Data::new(api_limiter.clone()))
            .app_data(Data::new(guest_limiter.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(fe_auth_handlers::configure_routes)
            .configure(fe_store_handlers::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
