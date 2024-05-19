use std::fmt::Error;
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

use crate::models::image::Image;
use crate::image_repository::schema::images::dsl::*;
type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_images(&self) -> Vec<Image> {
        images
            .load::<Image>(&mut self.pool.get().unwrap())
            .expect("Error loading image_repository")
    }

    pub fn add_image(&self, image: Image) -> Result<Image, Error> {
        let image = Image {
            id: uuid::Uuid::new_v4().to_string(),
            file_path: image.file_path,
            added_at: Utc::now().naive_utc(),
            taken_by: image.taken_by,
            taken_where: image.taken_where,
            notes: image.notes,
            ..image
        };
        diesel::insert_into(images)
            .values(&image)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error while adding new image to the database");
        Ok(image)
    }

    pub fn get_image_by_id(&self, image_id: &str) -> Option<Image> {
        let image = images
            .find(image_id)
            .get_result::<Image>(&mut self.pool.get().unwrap())
            .expect("Error loading image by id");
        Some(image)
    }

    pub fn delete_image_by_id(&self, image_id: &str) -> Option<usize> {
        let count = diesel::delete(images.find(image_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting image by id");
        Some(count)
    }


    pub fn update_image_by_id(&self, image_id: &str, image: Image) -> Option<Image> {
        let image = diesel::update(images.find(image_id))
            .set(&image)
            .get_result::<Image>(&mut self.pool.get().unwrap())
            .expect("Error updating image by id");
        Some(image)
    }
}