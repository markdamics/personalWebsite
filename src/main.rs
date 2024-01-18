#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

mod handlers;

// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    println!("Database URL: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::getAll)
            .service(handlers::add)
            .service(handlers::delete)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
