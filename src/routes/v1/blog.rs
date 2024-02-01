use crate::{
    errors::JsonError, extractors::authentication_token::AuthenticationToken, model::blog::Blog,
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
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, JsonError> {
    let post_id: Uuid = path.into_inner();

    match sqlx::query_as!(
        Blog,
        r#"
            SELECT * FROM blog_posts
            WHERE id = $1 
            LIMIT 1
        "#,
        post_id
    )
    .fetch_one(pool.get_ref())
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

#[get("")]
pub async fn get_blog_posts(pool: web::Data<PgPool>) -> Result<HttpResponse, JsonError> {
    match sqlx::query_as!(
        Blog,
        r#"
            SELECT * FROM blog_posts
        "#,
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to retrieve the requested records - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[post("/new")]
pub async fn post_blog_post(
    blog: web::Json<Blog>,
    pool: web::Data<PgPool>,
    _: AuthenticationToken,
) -> Result<HttpResponse, JsonError> {
    match sqlx::query_as!(
        Blog,
        r#"
            INSERT INTO blog_posts (id, author_id, title, body, last_updated, inserted_at)
            VALUES ($1, $2, $3, $4, $5, $6) 
        "#,
        Uuid::new_v4(),
        blog.author_id,
        blog.title,
        blog.body,
        Utc::now(),
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json("Success")),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to post the data requested - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
