mod routes;

use routes::test::{test, test_JSON, test_from_path, send_post};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Might need to add move to the closure
    HttpServer::new(|| {
        App::new()
           .service(test)
           .service(test_JSON)
           .service(test_from_path)
           .service(send_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
