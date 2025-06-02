#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Not found")]
    NotFound,
    #[error("Server error")]
    ServerError(Option<String>),
    #[error("Unexpected status {0}")]
    UnexpectedStatus(reqwest::StatusCode)
}
