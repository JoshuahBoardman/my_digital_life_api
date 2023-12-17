use actix_web::{HttpResponse, Result as ActixResult};

pub type Url = String;

pub trait CrudOperations {
    fn get_one(&self) -> ActixResult<HttpResponse>;
    fn get_all(&self) -> ActixResult<HttpResponse>;
    fn post_one(&self) -> ActixResult<HttpResponse>;
}
