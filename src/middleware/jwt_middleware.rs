
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub fn generate_jwt_token(
    user_uuid: &str,
    exp_secs: u64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expiration_time = current_time + exp_secs;

    let claims = Claims {
        sub: user_uuid.to_owned(),
        exp: expiration_time as usize,
        iat: current_time as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(dotenvy::var("APP_KEY").unwrap().as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt_token(
    token: &str,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    let validation = Validation {
        algorithms: vec![Algorithm::HS256],
        ..Validation::default()
    };

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(dotenvy::var("APP_KEY").unwrap().as_ref()),
        &validation,
    )?;
    Ok(token_data)
}


pub struct AuthorizationService {
    user_uuid: Option<String>,
}

impl AuthorizationService {
    pub fn uuid(&self) -> Option<&String> {
        self.user_uuid.as_ref()
    }
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            let secret = dotenvy::var("APP_KEY").unwrap();
            let token = auth_header.to_str().unwrap_or("");
            let decoding_key = DecodingKey::from_secret(secret.as_ref());

            match decode::<Claims>(token, &decoding_key, &Validation::default()) {
                Ok(token_data) => {
                    // Extract the 'sub' field from the token and store it in AuthorizationService
                    let user_uuid = token_data.claims.sub;
                    let auth_service = AuthorizationService {
                        user_uuid: Some(user_uuid),
                    };
                    ready(Ok(auth_service))
                }
                Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
            }
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("Missing token")))
        }
    }
}