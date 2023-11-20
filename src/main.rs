pub mod configuration;

use actix_web::web::Data;
use configuration::get_configuration;
use digital_bookshelf_api::run;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration_settings = get_configuration();
    let database_settings = &configuration_settings.database;
    let database_url = database_settings.get_database_url();
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to read configuration");
    let pool = Data::new(pool);

    run(configuration_settings.get_tcp_listener(), pool)?.await
}
