use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use handlers::frontend::*;
use lightning_cluster::{
    cluster::{Cluster, Node, NodeClient, NodeLightningImpl, NodeNetwork},
    lnd::LndClient,
};
use middleware::limiter_middleware::{ApiLimiter, GuestLimiter};
use moka::future::Cache;
use repositories::{
    checkout_repository::CheckoutRepository,
    donation_page_repository::{self, DonationPageRepository},
    store_repository::{StoreInvoiceRepository, StoreRepository},
    user_repository::UserRepository,
};
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
    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    let user_repo = UserRepository::new(pool.clone());
    let store_repo = StoreRepository::new(pool.clone());
    let store_invoice_repo = StoreInvoiceRepository::new(pool.clone());
    let checkout_repository = CheckoutRepository::new(pool.clone());
    let donation_page_repository = DonationPageRepository::new(pool.clone());
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(user_repo.clone()))
            .app_data(Data::new(store_repo.clone()))
            .app_data(Data::new(app_config.clone()))
            .app_data(Data::new(api_limiter.clone()))
            .app_data(Data::new(guest_limiter.clone()))
            .app_data(Data::new(store_invoice_repo.clone()))
            .app_data(Data::new(checkout_repository.clone()))
            .app_data(Data::new(donation_page_repository.clone()))
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(fe_auth_handlers::configure_routes)
            .configure(fe_store_handlers::configure_routes)
            .configure(fe_donation_page_handlers::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn init_cluster() -> Cluster {
    let node1 = Node {
        pubkey: dotenvy::var("NODE1_PUBKEY").unwrap(),
        ip: dotenvy::var("NODE1_IP").unwrap(),
        port: dotenvy::var("NODE1_PORT").unwrap(),
        network: NodeNetwork::Testnet,
        lightning_impl: NodeLightningImpl::Lnd,
        client: NodeClient::Lnd(LndClient::new(
            dotenvy::var("NODE1_HOST").unwrap(),
            dotenvy::var("NODE1_CERT_PATH").unwrap(),
            dotenvy::var("NODE1_MACAROON_PATH").unwrap(),
        )),
    };

    let nodes = vec![node1];
    let cluster = Cluster::new(nodes);

    cluster
}
