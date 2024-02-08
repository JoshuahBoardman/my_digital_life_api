use crate::{
    errors::JsonError,
    extractors::authentication_token::AuthenticationToken,
    model::{blog::Blog, common::RecordPagination},
    repository::blog::BlogRepository,
};
use actix_web::{get, post, web, HttpResponse, Scope};
use chrono::Utc;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub fn blog_scope() -> Scope {
    web::scope("/posts")
        .service(get_blog_posts)
        .service(get_blog_post)
        .service(post_blog_post)
}

#[get("/{id}")]
pub async fn get_blog_post(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, JsonError> {
    match Uuid::parse_str(path.into_inner().as_str()) {
        Ok(id) => {
            match BlogRepository::new(pool.as_ref())
                .fetch_book_by_id(id)
                .await
            {
                Ok(post) => Ok(HttpResponse::Ok().json(post)),
                Err(err) => Err(JsonError {
                    response_message: format!(
                        "Error: Failed to retrieve a record with the provided id - {}",
                        err
                    ),
                    error_code: StatusCode::INTERNAL_SERVER_ERROR,
                }),
            }
        }
        Err(err) => Err(JsonError {
            response_message: format!("Error: Invalid id - {}", err),
            error_code: StatusCode::BAD_REQUEST,
        }),
    }
}

#[get("")]
pub async fn get_blog_posts(
    pool: web::Data<PgPool>,
    record_pagination: web::Query<RecordPagination>,
) -> Result<HttpResponse, JsonError> {
    match BlogRepository::new(pool.as_ref())
        .fetch_blogs(
            record_pagination.limit.unwrap_or(25),
            record_pagination.offset.unwrap_or(0),
        )
        .await
    {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to retrieve the requested records - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[post("")]
pub async fn post_blog_post(
    blog: web::Json<Blog>,
    pool: web::Data<PgPool>,
    _: AuthenticationToken,
) -> Result<HttpResponse, JsonError> {
    match BlogRepository::new(pool.get_ref()).post_blog(&blog).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Success")),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to post the data requested - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
