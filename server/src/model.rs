// use std::time;

use diesel::prelude::*;

pub use super::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = houses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct House {
    pub name: String,
}

impl From<House> for shared::House {
    fn from(value: House) -> Self {
        Self { name: value.name }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<Room> for shared::Room {
    fn from(value: Room) -> Self {
        Self { id: value.id, name: value.name }
    }
}

#[derive(Insertable)]
#[diesel(table_name = rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRoom {
    pub name: String,
}

impl From<shared::NewRoom> for NewRoom {
    fn from(value: shared::NewRoom) -> Self {
        Self { name: value.name }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub name: String,
}

impl From<Device> for shared::Device {
    fn from(value: Device) -> Self {
        Self { id: value.id, room_id: value.room_id, name: value.name }
    }
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDevice {
    pub room_id: uuid::Uuid,
    pub name: String,
}


