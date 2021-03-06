//! Errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeascapeError {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("invalid response from jellyfin: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("attempted retrieval of empty token")]
    EmptyToken(),

    #[error("http error: {0}")]
    Http(#[from] http::Error),

    #[error("http client error: {0}")]
    HttpClient(#[from] crate::http::HttpClientError),

    #[error("header to_str error: {0}")]
    ToStr(#[from] http::header::ToStrError),

    #[error("invalid client configuration: {0}")]
    InvalidClient(String),

    #[error("unexpected data content type received")]
    InvalidContent,

    #[error("unknown jellyfin error")]
    Unknown,
}
