use crate::model::auth::Claims;
use actix_web::{
    error::ErrorUnauthorized, web, Error as ActixWebError, FromRequest, Result as ActixResult,
};
use jsonwebtoken::{
    decode, errors::Error as JWTError, Algorithm, DecodingKey, TokenData, Validation,
};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub claims: Claims,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
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
                return ready(Err(ErrorUnauthorized(
                    "Failed to find authentication cookie",
                )))
            }
        };

        let token = cookie.value();
        match validate_jwt(token, &secret) {
            Ok(claims) => ready(Ok(AuthenticationToken { claims: claims })),
            Err(err) => ready(Err(err)),
        }
    }
}

fn validate_jwt(jwt: &str, secret: &str) -> ActixResult<Claims> {
    let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    //if decode.

    match decode {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(ErrorUnauthorized(format!("JWT is invalid - {}", err))),
    }
}
