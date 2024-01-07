use actix_web::web::Data;
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::errors::JsonError;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub user_name: String, //TODO: Sanatize this
    pub email: String,     // TODO: Valideate this and Sanatize it
    pub inserted_at: DateTime<Utc>,
}

impl User {
    //TODO: merge all from_database associated functon into one, that consumes an enum
    pub async fn from_database_by_id(
        user_id: &Uuid,
        connection_pool: &Data<PgPool>,
    ) -> Result<Self, JsonError> {
        match sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE id = $1; 
        "#,
            user_id
        )
        .fetch_one(connection_pool.get_ref())
        .await
        {
            Ok(user) => Ok(user),
            Err(err) => {
                return Err(JsonError {
                    response_message: format!("User not found - {}", err).to_string(),
                    error_code: StatusCode::INTERNAL_SERVER_ERROR,
                })
            }
        }
    }

    pub async fn from_database_by_email(
        user_email: &str,
        connection_pool: &Data<PgPool>,
    ) -> Result<Self, JsonError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !(email_regex.is_match(&user_email)) {
            return Err(JsonError {
                response_message: "The email provided is invalid".to_string(),
                error_code: StatusCode::UNPROCESSABLE_ENTITY,
            });
        }
        match sqlx::query_as!(
            User,
            r#"
                SELECT * FROM 
                users WHERE email = $1
            "#,
            user_email
        )
        .fetch_one(connection_pool.get_ref())
        .await
        {
            Ok(user) => Ok(user),
            Err(err) => {
                return Err(JsonError {
                    response_message: format!("User not found - {}", err),
                    error_code: StatusCode::INTERNAL_SERVER_ERROR,
                })
            }
        }
    }
}
