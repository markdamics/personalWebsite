use actix_web::{delete, get, HttpResponse, post, put, web};
use crate::{models::image::Image, image_repository::database::Database};
#[get("/images")]
pub async fn get_images(db: web::Data<Database>) -> HttpResponse {
    let images = db.get_images();
    match images.as_slice() {
        [] => HttpResponse::NotFound().body("No images found"),
        _ => HttpResponse::Ok().json(images),
    }
}

#[get("/images/{image_id}")]
pub async fn get_image_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let image = db.get_image_by_id(&id);
    match image {
        Some(image) => HttpResponse::Ok().json(image),
        None => HttpResponse::NotFound().body("Image not found"),
    }
}

#[post("/images")]
pub async fn add_image(db: web::Data<Database>, image: web::Json<Image>) -> HttpResponse {
    let image = db.add_image(image.into_inner());
    match image {
        Ok(image) => HttpResponse::Ok().json(image),
        Err(err) => HttpResponse::InternalServerError().body("Error while adding new image to the database: ".to_owned() + &err.to_string()),
    }
}

#[delete("/images/{image_id}")]
pub async fn delete_image_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let image = db.delete_image_by_id(&id);
    match image {
        Some(image) => HttpResponse::Ok().json(image),
        None => HttpResponse::NotFound().body("Image not found"),
    }
}

#[put("/images/{image_id}")]
pub async fn update_image_by_id(db: web::Data<Database>, id: web::Path<String>, image: web::Json<Image>) -> HttpResponse {
    let image = db.update_image_by_id(&id, image.into_inner());
    match image {
        Some(image) => HttpResponse::Ok().json(image),
        None => HttpResponse::NotFound().body("Image not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_images)
            .service(get_image_by_id)
            .service(add_image)
            .service(delete_image_by_id)
            .service(update_image_by_id)
    );
}