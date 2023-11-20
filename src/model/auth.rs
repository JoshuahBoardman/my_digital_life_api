use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub exp: usize,
}
