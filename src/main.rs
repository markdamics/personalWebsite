use actix_web::{get, App, HttpResponse, HttpServer, Responder, post, delete, put, web};
use serde::Serialize;

mod handlers;

// async fn not_found() -> Result<HttpResponse> {
//     let response = Response {
//         message: "Resource not found".to_string(),
//     };
//     Ok(HttpResponse::NotFound().json(response))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(handlers::getAll)
            .service(handlers::add)
            .service(handlers::delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}