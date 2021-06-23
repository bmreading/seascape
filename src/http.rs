//! HTTP clients and related types

use async_trait::async_trait;
use std::collections::HashMap;
use thiserror::Error;

/// An HTTP Request type with a streaming body.
pub type Request = http::Request<String>;
pub type RequestBuilder = http::request::Builder;

/// An HTTP Response type with a streaming body.
pub type Response = http::Response<String>;

/// A map of parameters
pub type QueryParamMap<'a> = HashMap<&'a str, &'a str>;

/// Error type.
pub type Error = HttpClientError;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("http error: {0}")]
    Http(#[from] http::Error),

    #[error("jellyfin returned unauthorized error. check creds or token.")]
    Unauthorized,

    #[error("jellyfin returned forbidden error. check authorization level.")]
    Forbidden,

    #[error("unknown jellyfin error")]
    Unknown,
}

impl HttpClientError {
    fn from_status(code: u16) -> Self {
        if code == 401 {
            return Self::Unauthorized;
        } else if code == 403 {
            return Self::Forbidden;
        } else {
            return Self::Unknown;
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
            HttpClientType::AsyncClient(c) => Ok(c.send(&req, query_params).await?),
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
        let reqwest_response = self
            .backing_client
            .request(req.method().to_owned(), req.uri().to_string())
            .headers(req.headers().to_owned())
            .query(&query_params)
            .body(req.body().to_owned())
            .send()
            .await
            .unwrap();

        println!("Endpoint requested was: {}", reqwest_response.url());

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
    async fn into(self) -> http::Response<String>;
}

#[async_trait]
impl<T> IntoResponseExt<T> for reqwest::Response {
    async fn into(self) -> http::Response<String> {
        let mut http_response_builder: http::response::Builder = http::response::Builder::new();

        for header in self.headers() {
            http_response_builder = http_response_builder.header(header.0, header.1);
        }

        http_response_builder
            .status(self.status())
            .body(self.text().await.unwrap())
            .unwrap()
    }
}
