use actix_web::{web, get, HttpResponse, Responder, Result, web::Json, post};
use serde::{Serialize, Deserialize};

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
    tags: Vec<String>
}

#[post("/test/send_post")]
pub async fn send_post(body: Json<TestBlogPost>) -> Json<String> {
   Json(body.into_inner().body) 
}

