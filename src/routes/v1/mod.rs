use self::{auth::auth_scope, blog::blog_scope, books::book_shelf_scope};
use actix_web::{web, Scope};

pub mod auth;
pub mod blog;
pub mod books;

pub fn v1_scope() -> Scope {
    web::scope("/v1")
        .service(auth_scope())
        .service(blog_scope())
        .service(book_shelf_scope())
}
