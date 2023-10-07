use dotenvy::dotenv;
use std::{env, net::TcpListener};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

impl Settings {
    pub fn get_tcp_listener(&self) -> TcpListener {
        let address = format!("127.0.0.1:{}", self.port);
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

pub fn get_configuration() -> Settings {
    dotenv().expect("Error: unable to get .env variables");

    let db_user = &env::var("DB_USER").expect("Error: DB_USER .env variable not found");
    let db_password = &env::var("DB_PASSWORD").expect("Error: DB_PASSWORD .env variable not found");
    let db_port = &env::var("DB_PORT").expect("Error: DB_PORT .env variable not found");
    let db_host = &env::var("DB_HOST").expect("Error: DB_HOST .env variable not found");
    let db_name = &env::var("DB_NAME").expect("Error: DB_NAME .env variable not found");

    let port = &env::var("PORT").expect("Error: PORT .env variable not found");

    let database_settings = DatabaseSettings {
        user_name: db_user.to_string(),
        password: db_password.to_string(),
        port: db_port.parse::<u16>().unwrap(),
        host: db_host.to_string(),
        database_name: db_name.to_string(),
    };

    Settings {
        database: database_settings,
        port: port.parse::<u16>().unwrap(),
    }
}
