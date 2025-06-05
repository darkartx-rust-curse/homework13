use std::env;

use dotenv::dotenv;

use shared::*;

use client::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().unwrap();
    env_logger::init();

    let api_url = env::var("API_URL").unwrap();
    let client = Client::new(api_url).unwrap();

    let new_room1 = NewRoom { name: "Новая комната 1".to_string() };
    let new_room2 = NewRoom { name: "Новая комната 2".to_string() };

    let room1 = client.add_room(&new_room1).await.unwrap();
    let room2 = client.add_room(&new_room2).await.unwrap();

    println!("Room1: {room1:?}");
    println!("Room2: {room2:?}");

    let rooms = client.get_rooms().await.unwrap();
    println!("Rooms: {rooms:?}");

    let room_update = NewRoom { name: "Обновленная новая комната 1".to_string() };
    let room1 = client.update_room(room1.id, &room_update).await.unwrap();

    println!("Updated room: {room1:?}");

    let rooms = client.get_rooms().await.unwrap();
    println!("Rooms: {rooms:?}");

    client.delete_room(room2.id).await.unwrap();

    let rooms = client.get_rooms().await.unwrap();
    println!("Rooms: {rooms:?}");

    client.delete_room(room1.id).await.unwrap();

    let rooms = client.get_rooms().await.unwrap();
    println!("Rooms: {rooms:?}");
}
