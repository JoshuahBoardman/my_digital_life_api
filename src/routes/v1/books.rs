use crate::model::{book::Book, common::RecordPagination};
use crate::repository::book::BookRepository;
use crate::{errors::JsonError, extractors::authentication_token::AuthenticationToken};
use actix_web::{get, post, web, web::Json, HttpResponse, Result, Scope};
use chrono::Utc;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub fn book_shelf_scope() -> Scope {
    web::scope("/books")
        .service(get_book_by_id)
        .service(get_books)
        .service(post_book)
}

#[get("/{bookId}")]
pub async fn get_book_by_id(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, JsonError> {
    match Uuid::parse_str(path.into_inner().as_str()) {
        Ok(id) => {
            match BookRepository::new(pool.get_ref())
                .fetch_book_by_id(id)
                .await
            {
                Ok(book) => Ok(HttpResponse::Ok().json(book)),
                Err(err) => Err(JsonError {
                    response_message: format!("Error: Failed to fetch requested record - {}", err),
                    error_code: StatusCode::INTERNAL_SERVER_ERROR, //TODO: change error code set
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
pub async fn get_books(
    pool: web::Data<PgPool>,
    record_pagination: web::Query<RecordPagination>,
) -> Result<HttpResponse, JsonError> {
    match BookRepository::new(pool.get_ref())
        .fetch_books(
            record_pagination.limit.unwrap_or(25 as i64),
            record_pagination.offset.unwrap_or(0 as i64),
        )
        .await
    {
        Ok(books) => Ok(HttpResponse::Ok().json(books)),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to fetch requested records - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[post("")]
pub async fn post_book(
    book: Json<Book>,
    pool: web::Data<PgPool>,
    _: AuthenticationToken,
) -> Result<HttpResponse, JsonError> {
    match BookRepository::new(pool.get_ref()).post_book(&book).await {
        Ok(_) => Ok(HttpResponse::Ok().json("Success")),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to post the requested data - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
