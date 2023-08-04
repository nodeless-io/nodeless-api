use crate::{
    config::AppConfig,
    helpers::{
        crypto::sha256_hmac,
        format::ErrorResponse,
    },
    repositories::user_repository::UserRepository, middleware::jwt_middleware::generate_jwt_token,
};
use actix_web::{web, HttpResponse, Responder, ResponseError};
use serde::{Deserialize, Serialize};
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
    #[error("Login method is disabled.")]
    LoginMethodDisabled,
}

impl ResponseError for LoginError {
    fn error_response(&self) -> HttpResponse {
        match self {
            LoginError::InvalidEmail => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
            }),
            LoginError::PasswordTooShort => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
            }),
            LoginError::InvalidCredentials => HttpResponse::Unauthorized().json(ErrorResponse {
                error: self.to_string(),
            }),
            LoginError::WeakPassword => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
            }),
            LoginError::LoginMethodDisabled => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
            }),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JwtResponse {
    pub token: String,
}

pub async fn email(
    req: web::Json<EmailLoginReq>,
    user_repo: web::Data<UserRepository>,
    config: web::Data<AppConfig>,
) -> Result<HttpResponse, LoginError> {
    let login_data = req.into_inner();

    if !config.auth.enable_email_auth {
        return Err(LoginError::LoginMethodDisabled);
    }

    if !login_data.email.contains("@") {
        return Err(LoginError::InvalidEmail);
    }

    if login_data.password.len() < config.auth.min_password_length {
        return Err(LoginError::PasswordTooShort);
    }

    let hashed_pwd = sha256_hmac(&login_data.password, &dotenvy::var("APP_KEY").unwrap());
    let user = user_repo.get_user_by_email(&login_data.email).await;

    match user {
        Ok(user) => {
            if user.password != hashed_pwd {
                return Err(LoginError::InvalidCredentials);
            } else {
                let response = JwtResponse {
                    token: generate_jwt_token(&user.uuid, config.auth.jwt_expiry_seconds).unwrap(),
                };
                Ok(HttpResponse::Ok().json(response))
            }
        }
        Err(e) => {
            eprintln!("the error was {:?}", e);
            return Err(LoginError::InvalidCredentials);
        }
    }
}

// -----------------------------------------------------------------------------
// Nostr Login
// -----------------------------------------------------------------------------

pub struct NostrLogin {
    pub npub: String,
}

pub async fn nostr() -> impl Responder {
    HttpResponse::Ok().body("nostr_login")
}

// -----------------------------------------------------------------------------
// Identifier Login
// -----------------------------------------------------------------------------

pub async fn identifier() -> impl Responder {
    HttpResponse::Ok().body("identifier_login")
}

// -----------------------------------------------------------------------------
// Routes
// -----------------------------------------------------------------------------

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/email", web::post().to(email))
            .route("/nostr", web::post().to(nostr))
            .route("/identifier", web::post().to(identifier)),
    );
}
