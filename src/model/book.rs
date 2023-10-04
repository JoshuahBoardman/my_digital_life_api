use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Book {
    pub id: Option<Uuid>,
    pub name: String,
    pub author: String,
    pub description: String,
    pub rating: BigDecimal,
    pub review: String,
    pub finished: bool,
    pub inserted_at: Option<DateTime<Utc>>,
}

impl Book {
    pub fn new(
        name: String,
        author: String,
        description: String,
        rating: BigDecimal,
        review: String,
        finished: bool,
    ) -> Self {
        Book {
            id: None,
            name: name,
            author: author,
            description: description,
            rating: rating,
            review: review,
            finished: finished,
            inserted_at: None,
        }
    }
}
