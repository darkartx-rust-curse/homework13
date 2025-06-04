pub mod error;

use std::{fmt::Debug, result};

use reqwest::StatusCode;

pub use shared::*;

pub use error::Error;

pub type Result<T> = result::Result<T, Error>;

pub struct Client {
    api_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_url: String) -> Result<Self> {
        let client = reqwest::ClientBuilder::new().build()?;

        Ok(Self { api_url, client })
    }

    pub async fn get_house(&self) -> Result<House> {
        self.get("/").await
    }

    pub async fn get_rooms(&self) -> Result<Vec<Room>> {
        self.get("/rooms").await
    }

    pub async fn add_room<'a>(&self, new_room: &'a NewRoom) -> Result<Room> {
        self.post("/rooms", new_room).await
    }

    pub async fn get_room(&self, id: uuid::Uuid) -> Result<Room> {
        let path = format!("/rooms/{id}");
        self.get(&path).await
    }

    pub async fn update_room(&self, id: uuid::Uuid, room: &NewRoom) -> Result<Room> {
        let path = format!("/rooms/{id}");
        self.patch(&path, room).await
    }

    pub async fn get_devices(&self, room_id: String) -> Result<String> {
        let path = format!("/rooms/{room_id}/devices");
        self.get(&path).await
    }

    // pub async fn add_device(&self, room_id: String, name: String) -> Result<String> {
    //     let path = format!("/rooms/{room_id}/devices");
    //     self.post(&path).await
    // }

    pub async fn get_device(&self, room_id: String, id: String) -> Result<String> {
        let path = format!("/rooms/{room_id}/devices/{id}");
        self.get(&path).await
    }

    // pub async fn update_device(&self, room_id: String, id: String, name: String) -> Result<String> {
    //     let path = format!("/rooms/{room_id}/devices/{id}");
    //     self.patch(&path).await
    // }

    pub async fn get_report(&self) -> Result<String> {
        self.get("/report").await
    }

    async fn get<R: serde::de::DeserializeOwned>(&self, path: &str) -> Result<R> {
        let url = self.make_url(path);
        log::debug!("Request: GET {url}");

        let response = self.client.get(url).send().await?;
        log::debug!("Response: {response:?}");

        handle_response(response).await
    }

    async fn post<P, R>(&self, path: &str, payload: P) -> Result<R>
    where
        P: serde::ser::Serialize + Debug,
        R: serde::de::DeserializeOwned
    {
        let url = self.make_url(path);
        log::debug!("Request: POST {url} with {payload:?}");

        let response = self.client.post(url).json(&payload).send().await?;
        log::debug!("Response: {response:?}");

        handle_response(response).await
    }

    async fn patch<P, R>(&self, path: &str, payload: P) -> Result<R>
    where
        P: serde::ser::Serialize + Debug,
        R: serde::de::DeserializeOwned
    {
        let url = self.make_url(path);
        log::debug!("Request: PATCH {url} with {payload:?}");

        let response = self.client.patch(url).json(&payload).send().await?;
        log::debug!("Response: {response:?}");

        handle_response(response).await
    }

    fn make_url(&self, path: &str) -> String {
        format!("{}{path}", self.api_url)
    }
}

async fn handle_response<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED => response.json::<T>().await.map_err(Into::into),
        StatusCode::NOT_FOUND => Err(Error::NotFound),
        StatusCode::INTERNAL_SERVER_ERROR => {
            match response.json::<shared::Error>().await {
                Ok(error) => Err(Error::ServerError(Some(error.error))),
                Err(error) => Err(error.into())
            }
        },
        _ => Err(Error::UnexpectedStatus(response.status()))
    }
}
