use crate::{
    config,
    handlers::frontend::{fe_auth_handlers, fe_donation_page_handlers, fe_store_handlers},
    middleware::limiter_middleware::{ApiLimiter, GuestLimiter},
    models::user::User,
    repositories::{
        checkout_repository::CheckoutRepository,
        donation_page_repository::DonationPageRepository,
        store_repository::{StoreInvoiceRepository, StoreRepository},
        user_repository::{CreateUser, UserRepository, UserRepositoryError},
    },
};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware::Logger,
    web::Data,
    App, Responder,
};
use lightning_cluster::{
    cluster::{Cluster, Node, NodeClient, NodeLightningImpl, NodeNetwork},
    lnd::LndClient,
};
use moka::future::Cache;
use sqlx::PgPool;
use std::{fs::read_to_string, sync::Arc, time::Duration};
use toml::Value;

use super::{crypto::sha256_hmac, format::random_text};

pub async fn create_test_pool() -> PgPool {
    let pool = PgPool::connect(dotenvy::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();
    pool
}

pub async fn create_test_user() -> Result<User, UserRepositoryError> {
    let pool = create_test_pool().await;
    let user_repo = UserRepository::new(pool.clone());
    let random = random_text(10).await;
    let test_user = CreateUser {
        email: Some(String::from(format!("{}@nodeless.io", random))),
        password: Some(sha256_hmac(
            "password",
            dotenvy::var("APP_KEY").unwrap().as_str(),
        )),
        npub: None,
        identifier: None,
    };

    let user = user_repo.create(&test_user).await?;

    Ok(user)
}

pub async fn delete_test_user(uuid: &str) -> Result<bool, UserRepositoryError> {
    let pool = create_test_pool().await;
    let user_repo = UserRepository::new(pool.clone());

    let result = user_repo.hard_delete(uuid).await?;
    Ok(result)
}

pub async fn create_test_cluster() -> Cluster {
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

