// @generated automatically by Diesel CLI.

diesel::table! {
    houses (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
