pub mod configuration;
pub mod email_client;
pub mod extractors;
pub mod model;
mod routes;

use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use email_client::EmailClient;
use routes::auth::auth_scope;
use routes::books::book_shelf_scope;
use routes::health_check::health_check;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: Data<PgPool>,
    email_client: Data<EmailClient>,
) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .service(health_check)
            .service(book_shelf_scope())
            .service(auth_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
