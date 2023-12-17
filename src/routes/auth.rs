use crate::model::user::User;
use crate::{
    email_client::{EmailClient, TemplateModel},
    model::{
        auth::{Claims, LoginRequestBody, Secret, VerificationCode},
        common::Url,
    },
};
use actix_web::error::ErrorInternalServerError;
use actix_web::{
    cookie::{time, Cookie, SameSite},
    error::{Error as actix_error, ErrorUnauthorized},
    get, post, web, HttpResponse, Result, Scope,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::PgPool;
use uuid::Uuid;

pub fn auth_scope() -> Scope {
    web::scope("/auth").service(login).service(verify)
}

#[get("/verify/{code}")]
pub async fn verify(
    path: web::Path<String>,
    secret: web::Data<Secret>,
    pool: web::Data<PgPool>,
    base_url: web::Data<Url>,
) -> Result<HttpResponse, actix_error> {
    let user_verification_code: String = path.into_inner();

    let verification_code = match sqlx::query_as!(
        VerificationCode,
        "
                DELETE FROM verification_codes
                WHERE code = $1
                RETURNING *;
            ",
        user_verification_code
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(code) => code as VerificationCode,
        Err(err) => {
            return Err(ErrorInternalServerError(format!(
                "Invaild verification code - {}",
                err
            )))
        }
    };

    let verification_code_experation = verification_code.expires_at;

    let naive_current_time = Utc::now().naive_utc();

    if naive_current_time > verification_code_experation {
        return Err(ErrorUnauthorized(
            "The verification token provided has expired, please login to recieve a new token",
        ));
    }

    let user_record = match sqlx::query_as!(
        User,
        "
            SELECT * FROM users
            WHERE id = $1; 
        ",
        verification_code.user_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => user,
        Err(err) => {
            return Err(ErrorInternalServerError(format!(
                "Failed to find user record - {}",
                err
            )))
        }
    };

    let experation_timestamp = (Utc::now() + Duration::hours(1)).naive_utc();

    let claims = Claims {
        iss: base_url.as_str().to_string(),
        sub: user_record.id.to_owned(),
        iat: Utc::now().naive_utc(),
        exp: experation_timestamp.to_owned(),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.0.as_ref()),
    ) {
        Ok(t) => t,
        Err(err) => {
            return Err(ErrorInternalServerError(format!(
                "Failed to encode JWT - {}",
                err
            )))
        }
    };

    let cookie_duration = time::Duration::HOUR;

    let cookie = Cookie::build("authToken", token.to_owned())
        /*.domain("www.joshuahboardman-api.com/") // TODO: get a domain called
         * joshuahboardman-api for the api
        .path("/")*/
        .max_age(cookie_duration)
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(token)) //TODO dont send token
}

#[post("login")]
pub async fn login(
    email_client: web::Data<EmailClient>,
    pool: web::Data<PgPool>,
    req_body: web::Json<LoginRequestBody>,
    base_url: web::Data<Url>,
) -> Result<HttpResponse, actix_error> {
    let user_email: String = req_body.user_email.to_owned().to_string();

    // TODO: Make this a method on the user struct
    let user_record = match sqlx::query_as!(
        User,
        "
                SELECT id, user_name, email, inserted_at 
                FROM users WHERE email = $1
            ",
        user_email
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(user) => user as User,
        Err(err) => {
            return Err(ErrorInternalServerError(format!(
                "User not found - {}",
                err
            )))
        }
    };

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let inserted_at: DateTime<Utc> = Utc::now();

    let user_verificaton_code = VerificationCode {
        id: Uuid::new_v4(),
        user_id: user_record.id.to_owned(), //TODO: set this as the looked up user email GUID
        code: rand_string.to_owned(),
        expires_at: (inserted_at + Duration::hours(1)).naive_utc(),
        inserted_at,
    };

    //TODO: make this a method on the verificationcode struct
    //TODO: check if there is already a code and delete the previous one if there is
    let verification_code_result = sqlx::query!(
        r#"
            INSERT INTO verification_codes (id, code, expires_at, user_id, inserted_at) 
            VALUES ($1, $2, $3, $4, $5)
            "#,
        user_verificaton_code.id,
        user_verificaton_code.code,
        user_verificaton_code.expires_at,
        user_verificaton_code.user_id,
        user_verificaton_code.inserted_at,
    )
    .execute(pool.get_ref())
    .await;

    if let Err(err) = verification_code_result {
        return Err(ErrorInternalServerError(format!(
            "Failed to insert verifcation code - {}",
            err
        )));
    }

    let magic_link = format!("{}/auth/verify/{}", base_url.as_str(), rand_string);

    let template_model = TemplateModel {
        magic_link: magic_link.as_ref(),
        site_name: "JoshuahBoardman.com",
        user_name: user_record.user_name.as_str(),
    };

    let template_id = 34154243;

    match email_client
        .send_email(user_email, &template_id, "magic-link", &template_model)
        .await
    {
        Ok(json) => Ok(HttpResponse::Ok().json(format!("success, {}", json))),
        Err(error) => {
            println!("Issue sending email - {}", error);
            Err(ErrorUnauthorized(error.to_string()))
        }
    }
}
