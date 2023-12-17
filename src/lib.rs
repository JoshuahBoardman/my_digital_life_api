pub mod configuration;
pub mod email_client;
//pub mod extractors;
pub mod model;
mod routes;

use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use email_client::EmailClient;
use model::{auth::Secret, common::Url};
use routes::health_check::health_check;
use routes::{auth::auth_scope, books::book_shelf_scope};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    //TODO: pass base_url into app sate
    listener: TcpListener,
    db_pool: Data<PgPool>,
    email_client: Data<EmailClient>,
    secret: Data<Secret>,
    base_rul: Data<Url>,
) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(secret.clone())
            .app_data(base_rul.clone())
            .service(health_check)
            .service(book_shelf_scope())
            .service(auth_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
