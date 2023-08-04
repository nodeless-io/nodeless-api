use actix_web::{web, Error, HttpResponse, HttpRequest, Responder};
use futures::future::{ready, Ready};
use crate::middleware::jwt_middleware::AuthorizationService;
use crate::models::store::{Store};
use crate::repositories::store_repository::StoreRepository;
use crate::helpers::format::{DataResponse, ErrorResponse};

#[derive(Debug, serde::Deserialize)]
pub struct CreateStoreForm {
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateStoreForm {
    pub name: String,
}

pub async fn create_store(
    service: AuthorizationService,
    form: web::Json<CreateStoreForm>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = service.uuid().ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found")).unwrap();
    match repo.create(&user_uuid, &form.name).await {
        Ok(store) => HttpResponse::Created().json(DataResponse { data: store }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to create store".to_string() }),
    }
}

pub async fn get_all_stores(
    service: AuthorizationService,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = service.uuid().ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found")).unwrap();
    match repo.get_all(&user_uuid).await {
        Ok(stores) => HttpResponse::Ok().json(DataResponse { data: stores }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to get stores".to_string() }),
    }
}

pub async fn get_store_by_uuid(
    service: AuthorizationService,
    store_uuid: web::Path<String>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = service.uuid().ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found")).unwrap();
    match repo.get_by_uuid(&user_uuid, &store_uuid).await {
        Ok(Some(store)) => HttpResponse::Ok().json(DataResponse { data: store }),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse { error: "Store not found".to_string() }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to get store".to_string() }),
    }
}

pub async fn update_store(
    service: AuthorizationService,
    store_uuid: web::Path<String>,
    form: web::Json<UpdateStoreForm>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = service.uuid().ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found")).unwrap();
    match repo.update(&user_uuid, &store_uuid, &form.name).await {
        Ok(Some(updated_store)) => HttpResponse::Ok().json(DataResponse { data: updated_store }),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse { error: "Store not found".to_string() }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to update store".to_string() }),
    }
}

pub async fn delete_store(
    service: AuthorizationService,
    store_uuid: web::Path<String>,
    repo: web::Data<StoreRepository>,
) -> impl Responder {
    let user_uuid = service.uuid().ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found")).unwrap();
    match repo.delete(&user_uuid, &store_uuid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse { error: "Store not found".to_string() }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse { error: "Failed to delete store".to_string() }),
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
    );
}