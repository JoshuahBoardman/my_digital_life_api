use actix_web::{error::ErrorUnauthorized, http, web, Error as ActixWebError, FromRequest};
use jsonwebtoken::{
    decode, errors::Error as JWTError, Algorithm, DecodingKey, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use crate::model::auth::Claims;

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header: Option<&http::header::HeaderValue> =
            req.headers().get(http::header::AUTHORIZATION);
        let auth_token: String = auth_header
            .expect("Unable to extractrauthentication token")
            .to_str()
            .unwrap_or("")
            .to_string();
        if auth_token.is_empty() {
            return ready(Err(ErrorUnauthorized("Invalid auth token")));
        }

        let secret: String = req
            .app_data::<web::Data<String>>()
            .expect("Unable to retrieve the authentication secret")
            .to_string();

        let decode: Result<TokenData<Claims>, JWTError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unautherized"))),
        }
    }
}
