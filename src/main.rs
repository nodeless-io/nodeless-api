use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use config::AppConfig;
use handlers::frontend::fe_auth_handlers::configure_routes;
use repositories::{user_repository::UserRepository, store_repository::StoreRepository};
use sqlx::PgPool;
use std::fs::read_to_string;
use toml::Value;
use handlers::frontend::*;

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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(store_repo.clone()))
            .app_data(Data::new(app_config.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(fe_auth_handlers::configure_routes)
            .configure(fe_store_handlers::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
