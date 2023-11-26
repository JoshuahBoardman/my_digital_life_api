use crate::model::auth::{Claims, Secret};
use actix_web::{
    error::{Error as actix_error, ErrorUnauthorized},
    get, web, HttpResponse, Scope,
};
use jsonwebtoken::{
    decode, errors::Error as JWTError, Algorithm, DecodingKey, TokenData, Validation,
};

pub fn auth_scope() -> Scope {
    web::scope("/auth").service(verify)
    //.service(login)
}

// TODO: Verify Route - should check the url params for the token and store it in correct cookie
// storage.
#[get("/verify/{token}")]
pub async fn verify(
    path: web::Path<String>,
    secret: web::Data<Secret>,
) -> Result<HttpResponse, actix_error> {
    let token: String = path.into_inner();

    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let decode: Result<TokenData<Claims>, JWTError> = decode(
        &token, // TODO: token needs to contain both header and Claims
        &DecodingKey::from_secret(secret.0.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decode {
        Ok(token) => {
            //TODO: Set token to authentication header
            Ok(HttpResponse::Ok().json(token.claims))
        }
        Err(_) => Err(ErrorUnauthorized("Unautherized")),
    }
}

// TODO: Login Route - should take an email address as an input and send an email containing a link contains a jwt as
// a header if the email was registered.
