use crate::{errors::JsonError, model::auth::Claims};
use actix_web::{web, FromRequest};
use jsonwebtoken::{
    decode, errors::Error as JWTError, Algorithm, DecodingKey, TokenData, Validation,
};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub claims: Claims,
}

impl FromRequest for AuthenticationToken {
    type Error = JsonError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let secret: String = req
            .app_data::<web::Data<Secret<String>>>()
            .expect("Unable to retrieve the authentication secret")
            .expose_secret()
            .to_string();

        let cookie = match req.cookie("authToken") {
            Some(cookie) => cookie,
            None => {
                return ready(Err(JsonError {
                    response_message: "Failed to find authentication cookie".to_string(),
                    error_code: StatusCode::UNAUTHORIZED,
                }));
            }
        };

        let token = cookie.value();
        match validate_jwt(token, &secret) {
            Ok(claims) => ready(Ok(AuthenticationToken { claims: claims })),
            Err(err) => ready(Err(JsonError {
                response_message: format!("Error: Invalid authentication token - {}", err),
                error_code: StatusCode::UNAUTHORIZED,
            })),
        }
    }
}

fn validate_jwt(jwt: &str, secret: &str) -> Result<Claims, JWTError> {
    let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );
    //TODO: find out what the below comment was for... FeelsBadMan q_q
    //if decode.

    match decode {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err),
    }
}
