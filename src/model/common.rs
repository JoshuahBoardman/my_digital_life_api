use serde::{Deserialize, Serialize};

pub type Url = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordPagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
