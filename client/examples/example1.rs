use std::env;

use dotenv::dotenv;

use client::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().unwrap();
    env_logger::init();

    let api_url = env::var("API_URL").unwrap();
    let client = Client::new(api_url).unwrap();

    println!("get_house: {:?}", client.get_house().await.unwrap());
    // println!("get_rooms: {}", client.get_rooms().await.unwrap());
    // println!("add_room: {}", client.add_room("Test".to_string()).await.unwrap());
    // println!("get_room(1): {}", client.get_room("1".to_string()).await.unwrap());
    // println!("update_room(1): {}", client.update_room("1".to_string(), "Test2".to_string()).await.unwrap());
    // println!("get_devices(1): {}", client.get_devices("1".to_string()).await.unwrap());
    // println!("add_device(1): {}", client.add_device("1".to_string(), "Test".to_string()).await.unwrap());
    // println!("get_device(1, 1): {}", client.get_device("1".to_string(), "1".to_string()).await.unwrap());
    // println!("update_device(1, 1): {}", client.update_device("1".to_string(), "1".to_string(), "Test2".to_string()).await.unwrap());
    // println!("get_report: {}", client.get_report().await.unwrap());
}
