use crate::db;
use crate::error_handler::CustomError;
use crate::schema::images;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "images"]
pub struct Image {
    pub file_path: String,
}
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "images"]
pub struct Images {
    pub id: Uuid,
    pub file_path: String,
    pub added_at: NaiveDateTime,
}

impl Images {
    pub fn get_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let images = images::table.load::<Images>(&mut &conn)?;
        Ok(images)
    }

    pub fn create(image: Image) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut image = Image::from(image);
        image.id = Uuid::new_v4();
        image.added_at = Utc::now();
        let image = diesel::insert_into(images::table)
            .values(&image)
            .get_result(&mut &conn)?;
        Ok(image)
    }

    pub fn delete(id: String) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(images::table.filter(id.eq(&id))).execute(&conn)?;
        Ok(res)
    }
}
