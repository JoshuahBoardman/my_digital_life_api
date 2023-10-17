pub mod configuration;
pub mod model;
mod routes;

use configuration::get_configuration;

use routes::books::{get_book_by_id, get_books, post_book};
use routes::health_check::health_check;

use actix_web::{web, App, HttpServer};

//#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let configuration_settings = get_configuration();
    let database_settings = &configuration_settings.database;

    let database_url = database_settings.get_database_url();

    print!("{}", database_url);
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to read configuration.");
    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(health_check)
            .service(post_book)
            .service(get_book_by_id)
            .service(get_books)
    })
    .listen(configuration_settings.get_tcp_listener())?
    .run()
    .await
}
