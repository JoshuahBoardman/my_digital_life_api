use actix_web::{web, get, HttpResponse, Responder, Result};

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("testing123")
}

