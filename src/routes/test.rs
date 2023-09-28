use actix_web::{get, post, web, web::Json, HttpResponse, Responder, Result}; use
serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TestData {
    test_string: String,
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("testing123")
}

#[get("/test/JSON")]
pub async fn test_JSON() -> Json<String> {
    Json("testing in JSON".to_string())
}

#[get("/test/fromPath/{test_string}")]
pub async fn test_from_path(test_string: web::Path<TestData>) -> Json<String> {
    let test_string = test_string.into_inner().test_string;
    Json(format!("This is the test string: {}", test_string))
}

#[derive(Serialize, Deserialize)]
struct TestBlogPost {
    id: u32,
    title: String,
    body: String,
    tags: Vec<String>,
}

#[post("/test/send_post")]
pub async fn send_post(body: Json<TestBlogPost>) -> Json<String> {
    Json(body.into_inner().body)
}

#[derive(Deserialize, Serialize)]
pub struct Book {
    name: String,
    author: String,
    finished: bool,
}

//#[post("/test/post_book")]
pub async fn post_book(book: Json<Book>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
    INSERT INTO bookShelf (id, name, author, finished, added_at)
    VALUES ($1, $2, $3, $4, $5)
            "#,
        Uuid::new_v4(),
        book.name,
        book.author,
        book.finished,
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
