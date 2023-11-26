pub mod configuration;

use actix_web::web::Data;
use configuration::get_configuration;
use digital_bookshelf_api::{email_client::EmailClient, run};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration_settings = get_configuration();
    let database_settings = &configuration_settings.database;
    let database_url = &database_settings.get_database_url();

    let connection_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to read configuration");
    let connection_pool = Data::new(connection_pool);

    let application_settings = &configuration_settings.application;

    let email_client = EmailClient::new(
        application_settings.base_url,
        application_settings.sender_address,
        application_settings.authorization_token,
    );

    let email_client = Data::new(email_client);

    run(
        configuration_settings.get_tcp_listener(),
        connection_pool,
        email_client,
    )?
    .await
}
