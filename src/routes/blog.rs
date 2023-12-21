use crate::model::blog::Blog;
use actix_web::{
    error::ErrorInternalServerError, get, post, web, HttpResponse, Result as ActixResult, Scope,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub fn blog_scope() -> Scope {
    web::scope("/posts")
        .service(get_blog_post)
        .service(get_blog_posts)
        .service(post_blog_post)
}

#[get("/{id}")]
pub async fn get_blog_post(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> ActixResult<HttpResponse> {
    let post_id: Uuid = path.into_inner();

    match sqlx::query_as!(
        Blog,
        "
            SELECT * FROM blog_posts
            WHERE id = $1 
            LIMIT 1
        ",
        post_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(err) => Err(ErrorInternalServerError(format!(
            "Failed to retrieve a blog post with the provided id - {}",
            err
        ))),
    }
}

#[get("/")]
pub async fn get_blog_posts(pool: web::Data<PgPool>) -> ActixResult<HttpResponse> {
    match sqlx::query_as!(
        Blog,
        "
            SELECT * FROM blog_posts
        ",
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(err) => Err(ErrorInternalServerError(format!(
            "Failed to retrieve the requested blog posts - {}",
            err
        ))),
    }
}

#[post("/new")]
pub async fn post_blog_post(
    blog: web::Json<Blog>,
    pool: web::Data<PgPool>,
) -> ActixResult<HttpResponse> {
    match sqlx::query_as!(
        Blog,
        "
            INSERT INTO blog_posts (id, author_id, title, body, last_updated, inserted_at)
            VALUES ($1, $2, $3, $4, $5, $6) 
        ",
        Uuid::new_v4(),
        blog.author_id,
        blog.title,
        blog.body,
        Utc::now(),
        Utc::now() 
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(err) => Err(ErrorInternalServerError(format!(
            "Failed to post the blog post requested - {}",
            err
        ))),
    }
}
