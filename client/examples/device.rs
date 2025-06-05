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

    let new_device1 = NewDevice { name: "Холодильник".to_string() };
    let device1 = client.add_device(room1.id, &new_device1).await.unwrap();
    println!("Device 1 {device1:?}");

    let new_device2 = NewDevice { name: "Телевизор".to_string() };
    let device2 = client.add_device(room2.id, &new_device2).await.unwrap();
    println!("Device 2 {device2:?}");

    let room1_devices = client.get_devices(room1.id).await.unwrap();
    println!("Room 1 devices: {room1_devices:?}");

    let room2_devices = client.get_devices(room2.id).await.unwrap();
    println!("Room 2 devices: {room2_devices:?}");

    let new_device1 = NewDevice { name: "Холодильник 1".to_string() };
    client.update_device(room1.id, device1.id, &new_device1).await.unwrap();
    let device1 = client.get_device(room1.id, device1.id).await.unwrap();
    println!("Device 1 {device1:?}");

    client.delete_device(room2.id, device2.id).await.unwrap();

    let room2_devices = client.get_devices(room2.id).await.unwrap();
    println!("Room 2 devices: {room2_devices:?}");

    client.delete_room(room1.id).await.unwrap();
    client.delete_room(room2.id).await.unwrap();
}
