use crate::error_handler::CustomError;
use crate::images::{Image, Images};
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;

#[get("/getAllImages")]
async fn get_all() -> Result<HttpResponse, CustomError> {
    let images = Images::get_all()?;
    Ok(HttpResponse::Ok().json(images))
}

#[post("/addImage")]
async fn add(image: web::Json<Image>) -> Result<HttpResponse, CustomError> {
    let image = Images::create(image.into_inner());
    Ok(HttpResponse::Ok().json(image))
}

#[delete("/delete/{id}")]
async fn delete(id: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let deleted_image = Images::delete(id.into_inner());
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_image })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_all);
    config.service(add);
    config.service(delete);
}
