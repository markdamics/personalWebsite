use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::image_repository::schema::images)]
pub struct Image {
    #[serde(default)]
    pub id: String,
    pub file_path: String,
    pub added_at: NaiveDateTime,
    pub taken_by: Option<String>,
    pub taken_where: Option<String>,
    pub notes: Option<String>,
}