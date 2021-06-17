//! Interaction with Jellyfin

pub mod activity_log;
pub mod api_key;
pub mod artist;
pub mod builder;

pub use self::builder::Builder;

use http::header;
use serde_json::json;
use std::convert::TryFrom;

use crate::auth::{AuthHeader, AuthHeaderType, UserAuthHeader, UserAuthInfo};
use crate::error::JellyfinError;
use crate::http::{AsyncClient, HttpClient, HttpClientType};
use crate::model::{AuthenticationResult, BaseItemDtoQueryResult};

/// A type alias for a Result of generic type to be returned from a client instance
pub type ClientResult<T> = Result<T, JellyfinError>;

/// A type alias for a response from Jellyfin that wraps any number of base items
pub type ItemResponse<T> = BaseItemDtoQueryResult<T>;

/// The `Jellyfin` struct serves as the primary entrypoint for interacting with
/// the client API. By setting its `http_client_type` field, it is possible to
/// specify an asynchronous or blocking client. For practical usage, it is
/// recommended to use the `client::Builder` struct to create an instance.
#[derive(Debug)]
pub struct Jellyfin {
    http_client_type: HttpClientType,
    auth_header_type: Option<AuthHeaderType>,
    auth_result: Option<AuthenticationResult>,
    base_url: String,
}

impl Jellyfin {
    /// Constructs a new asynchronous Jellyfin client
    pub async fn new(base_url: String) -> ClientResult<Jellyfin> {
        let async_client = AsyncClient::default();
        let http_client_type = HttpClientType::AsyncClient(async_client);

        Ok(Jellyfin {
            http_client_type,
            auth_header_type: None,
            auth_result: None,
            base_url,
        })
    }

    /// Authenticates a Jellyfin client by means of user-based authentication
    pub async fn authenticate_by_user(
        self,
        username: &str,
        password: &str,
    ) -> ClientResult<Jellyfin> {
        let url = format!("{}{}", self.base_url, "/users/authenticatebyname");

        // TO DO: Look into seeing if this is better as a plain string or other type
        let body = json!({
            "username": username,
            "pw": password
        });

        let request = http::request::Request::builder()
            .uri(url.as_str())
            .method("POST")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .body(body.to_string())?;

        let response = self.http_client_type.send(&request, None).await?;

        let auth_result: AuthenticationResult =
            serde_json::from_str(response.body().to_owned().as_str())?;

        let user_auth_info_string = self
            .auth_header_type
            .as_ref()
            .unwrap()
            .header_value()
            .to_owned()
            .to_str()?
            .to_owned();

        let mut token_auth_info = UserAuthInfo::try_from(user_auth_info_string)?;

        // TO DO: Don't do clone
        token_auth_info.token = auth_result.access_token.clone();

        let token_auth_header = UserAuthHeader::new(token_auth_info);
        let token_auth_header_type = AuthHeaderType::UserAuthHeaderType(token_auth_header);

        Ok(Jellyfin {
            http_client_type: self.http_client_type,
            auth_header_type: Some(token_auth_header_type),
            auth_result: Some(auth_result),
            base_url: self.base_url,
        })
    }
}
