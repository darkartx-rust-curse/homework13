use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: uuid::Uuid,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRoom {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: uuid::Uuid,
    pub room_id: uuid::Uuid,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDevice {
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error: String
}
