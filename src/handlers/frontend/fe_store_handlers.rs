use crate::helpers::format::{DataResponse, ErrorResponse};
use crate::init_cluster;
use crate::middleware::jwt_middleware::AuthorizationService;
use crate::repositories::checkout_repository::CheckoutRepository;
use crate::repositories::store_repository::{StoreInvoiceRepository, StoreRepository};
use crate::services::checkout_service::{CheckoutService, CreateCheckoutService};
use crate::services::store_service::StoreService;
use actix_web::{web, HttpResponse, Responder};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateStoreReq {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStoreReq {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateStoreInvoice {
    pub amount: i64,
    pub expiry: i64,
    pub memo: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn create_store(
    auth: AuthorizationService,
    form: web::Json<CreateStoreReq>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.create(&user_uuid, &form.name).await {
        Ok(store) => HttpResponse::Created().json(DataResponse { data: store }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create store".to_string(),
        }),
    }
}

pub async fn get_all_stores(
    auth: AuthorizationService,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.get_all(&user_uuid).await {
        Ok(stores) => HttpResponse::Ok().json(DataResponse { data: stores }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to get stores".to_string(),
        }),
    }
}

pub async fn get_store_by_uuid(
    auth: AuthorizationService,
    store_uuid: web::Path<String>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.get_by_uuid(&user_uuid, &store_uuid).await {
        Ok(Some(store)) => HttpResponse::Ok().json(DataResponse { data: store }),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Store not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to get store".to_string(),
        }),
    }
}

pub async fn update_store(
    auth: AuthorizationService,
    store_uuid: web::Path<String>,
    form: web::Json<UpdateStoreReq>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.update(&user_uuid, &store_uuid, &form.name).await {
        Ok(Some(updated_store)) => HttpResponse::Ok().json(DataResponse {
            data: updated_store,
        }),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Store not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to update store".to_string(),
        }),
    }
}

pub async fn delete_store(
    auth: AuthorizationService,
    store_uuid: web::Path<String>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.delete(&user_uuid, &store_uuid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Store not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to delete store".to_string(),
        }),
    }
}

pub async fn create_store_invoice(
    auth: AuthorizationService,
    data: web::Json<CreateStoreInvoice>,
    store_uuid: web::Path<String>,
    store_repo: web::Data<StoreRepository>,
    checkout_repo: web::Data<CheckoutRepository>,
    store_invoice_repo: web::Data<StoreInvoiceRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();

    let service = StoreService::new(
        store_repo.get_ref().clone(),
        checkout_repo.get_ref().clone(),
        store_invoice_repo.get_ref().clone(),
    );

    let invoice = service
        .create_invoice(
            &store_uuid,
            data.clone().metadata,
            CreateCheckoutService {
                user_uuid: user_uuid.to_string(),
                amount: data.amount,
                expiry: data.expiry,
                memo: data.memo.clone(),
            },
            CheckoutService::new(init_cluster().await),
        )
        .await;

    match invoice {
        Ok(invoice) => HttpResponse::Created().json(DataResponse { data: invoice }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: e.to_string(),
        }),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stores")
            .route("", web::post().to(create_store))
            .route("", web::get().to(get_all_stores))
            .route("/{store_uuid}", web::get().to(get_store_by_uuid))
            .route("/{store_uuid}", web::put().to(update_store))
            .route("/{store_uuid}", web::delete().to(delete_store))
            .route(
                "/{store_uuid}/invoices",
                web::post().to(create_store_invoice),
            ),
    );
}
