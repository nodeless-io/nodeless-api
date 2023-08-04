use actix_web::{Responder, HttpResponse, web, ResponseError};
use serde::{Deserialize, Serialize};
use crate::{helpers::{format::{ErrorResponse, DataResponse}, crypto::sha256_hmac}, repositories::user_repository::UserRepository};
use thiserror::Error;

// -----------------------------------------------------------------------------
// Email Login
// -----------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct EmailLoginReq {
    pub email: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Password too short")]
    PasswordTooShort,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Password isn't strong enough")]
    WeakPassword,
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LoginError::InvalidEmail => HttpResponse::BadRequest().json(ErrorResponse { error: self.to_string() }),
            LoginError::PasswordTooShort => HttpResponse::BadRequest().json(ErrorResponse { error: self.to_string() }),
            LoginError::InvalidCredentials => HttpResponse::Unauthorized().json(ErrorResponse { error: self.to_string() }),
            LoginError::WeakPassword => HttpResponse::BadRequest().json(ErrorResponse { error: self.to_string() }),
        }
    }
}

pub async fn email(req: web::Json::<EmailLoginReq>, user_repo: web::Data::<UserRepository>) -> Result<HttpResponse, LoginError> {
    let login_data = req.into_inner();

    if !login_data.email.contains("@") {
        return Err(LoginError::InvalidEmail);
    }

    if login_data.password.len() < 8 {
        return Err(LoginError::PasswordTooShort);
    }

    let hashed_pwd = sha256_hmac(&login_data.password, &dotenvy::var("APP_KEY").unwrap());
    let user = user_repo.get_user_by_email(&login_data.email).await;

    match user {
        Ok(user) => {
            if user.password != hashed_pwd {
                eprintln!("the user password was {} and the hashed password was {}", user.password, hashed_pwd);
                return Err(LoginError::InvalidCredentials);
            }
        }
        Err(e) => {
            eprintln!("the error was {:?}", e);
            return Err(LoginError::InvalidCredentials);
        }
    }

    let response = DataResponse { data: login_data };
    Ok(HttpResponse::Ok().json(response))
}

pub struct NostrLogin {
    pub npub: String,
}

pub async fn nostr() -> impl Responder {
    HttpResponse::Ok().body("nostr_login")
}

pub async fn identifier() -> impl Responder {
    HttpResponse::Ok().body("identifier_login")
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/email", web::post().to(email))
            .route("/nostr", web::post().to(nostr))
            .route("/identifier", web::post().to(identifier)),
    );
}