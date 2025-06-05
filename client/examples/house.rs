use std::env;

use dotenv::dotenv;

use client::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().unwrap();
    env_logger::init();

    let api_url = env::var("API_URL").unwrap();
    let client = Client::new(api_url).unwrap();

    let house = client.get_house().await.unwrap();

    println!("House: {house:?}");
}
