use std::time;

use diesel::prelude::*;

diesel::table! {
    houses (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = houses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct House {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: time::SystemTime,
    pub updated_at: time::SystemTime,
}
