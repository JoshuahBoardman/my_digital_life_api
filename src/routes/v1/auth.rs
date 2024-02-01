use crate::{
    email_client::{EmailClient, EmailTemplate, TemplateModel},
    errors::JsonError,
    model::{
        auth::{Claims, LoginRequestBody, /*Secret,*/ VerificationCode},
        common::Url,
        user::User,
    },
};
use actix_web::{
    cookie::{time, Cookie, SameSite},
    post, web, HttpResponse, Scope,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use uuid::Uuid;

pub fn auth_scope() -> Scope {
    web::scope("/auth").service(login).service(verify)
}

#[post("/verify/{code}")]
pub async fn verify(
    path: web::Path<String>,
    secret: web::Data<Secret<String>>,
    pool: web::Data<PgPool>,
    base_url: web::Data<Url>,
) -> Result<HttpResponse, JsonError> {
    let user_verification_code: String = path.into_inner();

    let verification_code = VerificationCode::from_database(&user_verification_code, &pool).await?;
    verification_code.verify()?;

    let user_record = User::from_database_by_id(&verification_code.user_id, &pool).await?;

    let experation_timestamp = (Utc::now() + Duration::hours(1)).naive_utc();

    let claims = Claims {
        iss: base_url.as_str().to_string(),
        sub: user_record.id.to_owned(),
        iat: Utc::now().naive_utc(),
        exp: experation_timestamp.timestamp().to_owned() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.expose_secret().as_ref()),
    ) {
        Ok(jwt) => jwt,
        Err(err) => {
            return Err(JsonError {
                response_message: format!("Error: Failed to encode JWT - {}", err),
                error_code: StatusCode::INTERNAL_SERVER_ERROR,
            })
        }
    };

    let cookie_duration = time::Duration::HOUR;

    let cookie = Cookie::build("authToken", token.to_owned())
        //.domain(base_url.as_str())
        .path("/")
        .max_age(cookie_duration)
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json("Success"))
}

#[post("login")]
pub async fn login(
    email_client: web::Data<EmailClient>,
    email_template: web::Data<EmailTemplate>,
    pool: web::Data<PgPool>,
    req_body: web::Json<LoginRequestBody>,
    base_url: web::Data<Url>,
) -> Result<HttpResponse, JsonError> {
    let user_email: String = req_body.user_email.to_owned().to_string();

    let user_record = User::from_database_by_email(&user_email, &pool).await?;

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let inserted_at: DateTime<Utc> = Utc::now();

    let user_verificaton_code = VerificationCode {
        id: Uuid::new_v4(),
        user_id: user_record.id.to_owned(),
        code: rand_string.to_owned(),
        expires_at: (inserted_at + Duration::hours(1)).naive_utc(),
        inserted_at,
    };

    user_verificaton_code.post_in_database(&pool).await?;

    let magic_link = format!("{}/v1/auth/verify/{}", base_url.as_str(), rand_string);

    let template_model = TemplateModel {
        magic_link: magic_link.as_ref(),
        user_name: user_record.user_name.as_str(),
    };

    match email_client
        .send_email(
            &user_email,
            &email_template.template_id,
            &email_template.template_alias,
            &template_model,
        )
        .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json("Success")),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Issue sending email - {}", err),
            error_code: StatusCode::UNAUTHORIZED,
        }),
    }
}
