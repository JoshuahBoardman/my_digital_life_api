use dotenvy::dotenv;
use std::{env, net::TcpListener};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub secret: String, // TODO: may want to add something like secrecy to obscure the secret (keep
                        // things tight)
}

impl ApplicationSettings {
    pub fn get_tcp_listener(&self) -> TcpListener {
        let address = format!("{}:{}", self.host, self.port);
        TcpListener::bind(address).expect("Error: TcpListener failed")
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub user_name: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user_name, self.password, self.host, self.port, self.database_name
        )
    }
}

#[derive(serde::Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_address: String,
    pub authorization_token: String,
}

pub fn get_configuration() -> Settings {
    dotenv().expect("Error: unable to get .env variables");

    let db_user = &env::var("DB_USER").expect("Error: DB_USER .env variable not found");
    let db_password = &env::var("DB_PASSWORD").expect("Error: DB_PASSWORD .env variable not found");
    let db_port = &env::var("DB_PORT").expect("Error: DB_PORT .env variable not found");
    let db_host = &env::var("DB_HOST").expect("Error: DB_HOST .env variable not found");
    let db_name = &env::var("DB_NAME").expect("Error: DB_NAME .env variable not found");

    let application_host =
        &env::var("APPLICATION_HOST").expect("Error: APPLICATION_HOST .env variable not found");
    let application_port =
        &env::var("APPLICATION_PORT").expect("Error: APPLICATION_PORT .env variable not found");
    let application_base_url = &env::var("APPLICATION_BASE_URL")
        .expect("Error: APPLICATION_BASE_URL .env variable not found");
    let application_secret =
        &env::var("APPLICATION_SECRET").expect("Error: APPLICATION_SECRET .env variable not found");

    let email_client_base_url = &env::var("EMAIL_CLIENT_BASE_URL")
        .expect("Error: EMAIL_CLIENT_BASE_URL .env variable not found");
    let email_client_sender_address = &env::var("EMAIL_CLIENT_SENDER_ADDRESS")
        .expect("Error: EMAIL_CLIENT_SENDER_ADDRESS .env variable not found");
    let email_client_authorization_token = &env::var("EMAIL_CLIENT_AUTHORIZATION_TOKEN")
        .expect("Error: EMAIL_CLIENT_SENDER_ADDRESS .env variable not found");

    let database_settings = DatabaseSettings {
        user_name: db_user.to_string(),
        password: db_password.to_string(),
        port: db_port.parse::<u16>().expect("Erorr: value is not a u16"),
        host: db_host.to_string(),
        database_name: db_name.to_string(),
    };

    let application_settings = ApplicationSettings {
        host: application_host.to_string(),
        port: application_port
            .parse::<u16>()
            .expect("Error: value is not a u16"),
        base_url: application_base_url.to_string(),
        secret: application_secret.to_string(),
    };

    let email_client = EmailClientSettings {
        base_url: email_client_base_url.to_string(),
        sender_address: email_client_sender_address.to_string(),
        authorization_token: email_client_authorization_token.to_string(),
    };

    Settings {
        database: database_settings,
        application: application_settings,
        email_client: email_client,
    }
}
