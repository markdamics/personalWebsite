// @generated automatically by Diesel CLI.

diesel::table! {
    images (id) {
        id -> Int4,
        #[max_length = 140]
        filepath -> Varchar,
        added_at -> Timestamp,
    }
}
