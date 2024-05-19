use diesel::table;

table! {
    images (id) {
        id -> Varchar,
        file_path -> Varchar,
        added_at -> Timestamp,
        taken_by -> Nullable<Varchar>,
        taken_where -> Nullable<Varchar>,
        notes -> Nullable<Text>,
    }
}// @generated automatically by Diesel CLI.

