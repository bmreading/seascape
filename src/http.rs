//! HTTP clients and related types

use async_trait::async_trait;
use bytes::Bytes;
use log::info;
use std::collections::HashMap;
use thiserror::Error;

/// An HTTP Request type
pub type Request = http::Request<Option<String>>;
pub type RequestBuilder = http::request::Builder;

/// An HTTP Response type
pub type Response = http::Response<DataContentType>;

/// A map of parameters
pub type QueryParamMap<'a> = HashMap<&'a str, &'a str>;

/// Error type.
pub type Error = HttpClientError;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("http error: {0}")]
    Http(#[from] http::Error),

    #[error("jellyfin returned bad request error. check parameters.")]
    BadRequest,

    #[error("jellyfin returned unauthorized error. check creds or token.")]
    Unauthorized,

    #[error("jellyfin returned forbidden error. check authorization level.")]
    Forbidden,

    #[error("jellyfin returned not found error. check parameters.")]
    NotFound,

    #[error("unknown jellyfin error")]
    Unknown,
}

impl HttpClientError {
    fn from_status(code: u16) -> Self {
        if code == 400 {
            Self::BadRequest
        } else if code == 401 {
            Self::Unauthorized
        } else if code == 403 {
            Self::Forbidden
        } else if code == 404 {
            Self::NotFound
        } else {
            Self::Unknown
        }
    }
}

#[async_trait]
pub trait HttpClient: std::fmt::Debug + Unpin + Send + Sync + 'static {
    /// Perform a request.
    async fn send(
        &self,
        req: &Request,
        query_params: Option<&QueryParamMap>,
    ) -> Result<Response, Error>;
}

#[derive(Debug, Clone)]
pub enum HttpClientType {
    AsyncClient(AsyncClient),
}

#[async_trait]
impl HttpClient for HttpClientType {
    async fn send(
        &self,
        req: &Request,
        query_params: Option<&QueryParamMap>,
    ) -> Result<Response, Error> {
        match self {
            HttpClientType::AsyncClient(c) => Ok(c.send(req, query_params).await?),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AsyncClient {
    backing_client: reqwest::Client,
}

impl AsyncClient {
    async fn send(
        &self,
        req: &Request,
        query_params: Option<&QueryParamMap<'_>>,
    ) -> Result<Response, Error> {
        let body = if let Some(b) = req.body() { b } else { "" };

        let reqwest_response = self
            .backing_client
            .request(req.method().to_owned(), req.uri().to_string())
            .headers(req.headers().to_owned())
            .query(&query_params)
            .body(body.to_owned())
            .send()
            .await
            .unwrap();

        info!("Endpoint requested was: {}", reqwest_response.url());

        if reqwest_response.status().is_success() {
            Ok(IntoResponseExt::<reqwest::Response>::into(reqwest_response).await)
        } else {
            Err(HttpClientError::from_status(
                reqwest_response.status().as_u16(),
            ))
        }
    }
}

#[async_trait]
pub trait IntoResponseExt<T> {
    async fn into(self) -> http::Response<DataContentType>;
}

#[async_trait]
impl<T> IntoResponseExt<T> for reqwest::Response {
    // From reqwest::Response to http::Response<DataContentType>
    async fn into(self) -> http::Response<DataContentType> {
        let mut http_response_builder = http::response::Builder::new().status(self.status());

        for header in self.headers() {
            http_response_builder = http_response_builder.header(header.0, header.1);
        }

        const TEXT_DATA_CONTENT_TYPES: [&str; 2] = ["application/json", "application/xhtml"];

        let mut is_text_data = false;
        let mut is_binary_data = false;
        for header in self.headers() {
            http_response_builder = http_response_builder.header(header.0, header.1);

            // Determine if binary or text data based-on Content-Type header
            if (header.0.as_str() == "content-type" && header.1.to_str().unwrap().contains("text"))
                || (header.0.as_str() == "content-type"
                    && TEXT_DATA_CONTENT_TYPES
                        .iter()
                        .any(|v| header.1.to_str().unwrap().contains(v)))
            {
                is_text_data = true;
            } else if header.0.as_str() == "content-type"
                && !header.1.to_str().unwrap().contains("text")
            {
                is_binary_data = true;
            }
        }

        if is_text_data {
            http_response_builder
                .body(DataContentType::TextContent(self.text().await.unwrap()))
                .unwrap()
        } else if is_binary_data {
            http_response_builder
                .body(DataContentType::BinaryContent(self.bytes().await.unwrap()))
                .unwrap()
        } else {
            http_response_builder
                .body(DataContentType::NoContent)
                .unwrap()
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataContentType {
    TextContent(String),
    BinaryContent(Bytes),
    NoContent,
}
