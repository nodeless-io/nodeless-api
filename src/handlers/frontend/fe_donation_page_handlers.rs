use crate::helpers::format::{DataResponse, ErrorResponse};
use crate::middleware::jwt_middleware::AuthorizationService;
use crate::models::donation_page;
use crate::repositories::donation_page_repository::{
    CreateDonationPage, DonationPageRepository, RepoError, UpdateDonationPage,
};
use actix_web::{web, HttpResponse, Responder};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateDonationPageReq {
    pub slug: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDonationPageReq {
    pub slug: String,
    pub name: String,
    pub description: String,
}

pub async fn create_donation_page(
    auth: AuthorizationService,
    form: web::Json<CreateDonationPageReq>,
    repo: web::Data<DonationPageRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();

    let donation_page = CreateDonationPage {
        user_uuid: user_uuid.to_string(),
        slug: form.slug.clone(),
        name: form.name.clone(),
        description: form.description.clone(),
    };

    match repo.create(donation_page).await {
        Ok(donation_page) => HttpResponse::Created().json(DataResponse {
            data: donation_page,
        }),
        Err(RepoError::SlugTaken) => HttpResponse::Conflict().json(ErrorResponse {
            error: "Slug already taken".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to create donation page".to_string(),
        }),
    }
}

pub async fn get_all_donation_pages(
    auth: AuthorizationService,
    repo: web::Data<DonationPageRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.get_all_by_user_uuid(&user_uuid).await {
        Ok(donation_pages) => HttpResponse::Ok().json(DataResponse {
            data: donation_pages,
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to get donation pages".to_string(),
        }),
    }
}

pub async fn get_one_by_uuid(
    auth: AuthorizationService,
    donation_page_uuid: web::Path<String>,
    repo: web::Data<DonationPageRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo
        .get_one_by_user_uuid(&user_uuid, &donation_page_uuid)
        .await
    {
        Ok(donation_page) => HttpResponse::Ok().json(DataResponse {
            data: donation_page,
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to get donation page".to_string(),
        }),
    }
}

pub async fn update_donation_page(
    auth: AuthorizationService,
    donation_page_uuid: web::Path<String>,
    form: web::Json<UpdateDonationPageReq>,
    repo: web::Data<DonationPageRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();

    let update_donation_page = UpdateDonationPage {
        slug: form.slug.clone(),
        name: form.name.clone(),
        description: form.description.clone(),
    };

    match repo
        .update_by_user_uuid(&donation_page_uuid, &user_uuid, update_donation_page)
        .await
    {
        Ok(donation_page) => HttpResponse::Ok().json(DataResponse {
            data: donation_page,
        }),
        Err(RepoError::SlugTaken) => HttpResponse::Conflict().json(ErrorResponse {
            error: "Slug already taken".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to update donation page".to_string(),
        }),
    }
}

pub async fn delete_donation_page(
    auth: AuthorizationService,
    donation_page_uuid: web::Path<String>,
    repo: web::Data<DonationPageRepository>,
) -> impl Responder {
    let user_uuid = auth
        .uuid()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User UUID not found"))
        .unwrap();
    match repo.delete_by_user(&donation_page_uuid, &user_uuid).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Donation page not found".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: "Failed to delete donation page".to_string(),
        }),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/donation-pages")
            .route("", web::post().to(create_donation_page))
            .route("", web::get().to(get_all_donation_pages))
            .route("/{donation_page_uuid}", web::get().to(get_one_by_uuid))
            .route("/{donation_page_uuid}", web::put().to(update_donation_page))
            .route(
                "/{donation_page_uuid}",
                web::delete().to(delete_donation_page),
            ),
    );
}
