// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Uuid,
        #[max_length = 255]
        file_path -> Varchar,
        added_at -> Timestamp,
    }
}
