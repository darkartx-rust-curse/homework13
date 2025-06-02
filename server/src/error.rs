use std::error;

use axum::{response::{Response, IntoResponse, Json}, http::StatusCode};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Internal(Box<dyn error::Error>),
    #[error("House not found")]
    HouseNotFound
}

impl Error {
    pub fn from_internal<T: error::Error + 'static>(error: T) -> Self {
        Self::Internal(Box::new(error))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let error = shared::Error {
            error: self.to_string()
        };

        match self {
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error)),
            Self::HouseNotFound => (StatusCode::NOT_FOUND, Json(error))
        }.into_response()
    }
}