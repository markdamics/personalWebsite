use actix_web::{get, App, HttpResponse, HttpServer, Responder, post, delete, put, web};

#[get("/getAll")]
async fn getAll() -> impl Responder {
    HttpResponse::Ok().body("All images are loaded from the database")
}

#[post("/add/{filePath}")]
async fn add(filePath: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Image({}) added to the database", filePath))
}

#[delete("/delete/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Image({}) removed from the database", id))
}