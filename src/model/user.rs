use actix_web::{error, web::Data, Result as ActixRsult};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

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
    ) -> ActixRsult<Self> {
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
                return Err(error::ErrorInternalServerError(format!(
                    "User not found - {}",
                    err
                )))
            }
        }
    }

    pub async fn from_database_by_email(
        user_email: &str,
        connection_pool: &Data<PgPool>,
    ) -> ActixRsult<Self> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

        if !(email_regex.is_match(&user_email)) {
            return Err(error::ErrorUnprocessableEntity(
                "The email provided is invalid",
            ));
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
                return Err(error::ErrorInternalServerError(format!(
                    "User not found - {}",
                    err
                )))
            }
        }
    }
}
