// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Uuid,
        room_id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    houses (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rooms (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(devices -> rooms (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    houses,
    rooms,
);
