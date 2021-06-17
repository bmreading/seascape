//! Used for authenticating with Jellyfin

use core::fmt;
use http::header::{HeaderName, HeaderValue};
use std::collections::HashMap;
use std::{convert::TryFrom, str::FromStr};

use crate::error::{self, SeascapeError};

/// The HTTP header name of a Jellyfin user-based authentication
pub const USER_AUTH_HEADER_KEY: &str = "X-Emby-Authorization";

/// The HTTP header name of a Jellyfin API key-based authentication
pub const API_KEY_AUTH_HEADER_KEY: &str = "X-Emby-Token";

/// An HTTP header used for authenticating with Jellyfin.
pub trait AuthHeader {
    fn header_key_name(&self) -> &HeaderName;
    fn header_value(&self) -> &HeaderValue;
}

/// Holds variants of headers that are used for authenticating with Jellyfin.
#[derive(Debug)]
pub enum AuthHeaderType {
    UserAuthHeaderType(UserAuthHeader),
    ApiKeyAuthHeaderType(ApiKeyAuthHeader),
}

impl AuthHeader for AuthHeaderType {
    fn header_key_name(&self) -> &HeaderName {
        match self {
            AuthHeaderType::UserAuthHeaderType(h) => &h.header_key_name,
            AuthHeaderType::ApiKeyAuthHeaderType(h) => &h.header_key_name,
        }
    }

    fn header_value(&self) -> &HeaderValue {
        match self {
            AuthHeaderType::UserAuthHeaderType(h) => &h.header_value,
            AuthHeaderType::ApiKeyAuthHeaderType(h) => &h.header_value,
        }
    }
}

/// The details of a header used for user-based authentication with Jellyfin.
#[derive(Debug, Clone)]
pub struct UserAuthHeader {
    header_key_name: HeaderName,
    header_value: HeaderValue,
}

impl UserAuthHeader {
    /// Return a new UserAuthHeader
    pub fn new(auth_info: UserAuthInfo) -> Self {
        let header_key_name = match HeaderName::from_str(USER_AUTH_HEADER_KEY) {
            Ok(n) => n,
            Err(_) => HeaderName::from_str("test").unwrap(),
        };

        UserAuthHeader {
            header_key_name,
            header_value: HeaderValue::from_str(auth_info.to_string().as_str()).unwrap(),
        }
    }
}

/// The details of a header used for api key authentication with Jellyfin.
#[derive(Debug, Clone)]
pub struct ApiKeyAuthHeader {
    pub header_key_name: HeaderName,
    pub header_value: HeaderValue,
}

impl ApiKeyAuthHeader {
    /// Return a new ApiKeyAuthHeader
    pub fn new(api_key: &str) -> Self {
        ApiKeyAuthHeader {
            header_key_name: HeaderName::from_static(API_KEY_AUTH_HEADER_KEY),
            header_value: HeaderValue::from_str(api_key).unwrap(),
        }
    }
}

/// Data used for creating and using a session with Jellyfin when using user-based authentication
#[derive(Debug)]
pub struct UserAuthInfo {
    client: String,
    device: String,
    device_id: String,
    version: String,
    pub token: Option<String>,
}

impl UserAuthInfo {
    pub fn new(
        client: &str,
        device: &str,
        device_id: &str,
        version: &str,
        token: Option<&str>,
    ) -> Self {
        UserAuthInfo {
            client: client.to_string(),
            device: device.to_string(),
            device_id: device_id.to_string(),
            version: version.to_string(),
            token: token.map(String::from),
        }
    }
}

// We want to use this as a string
impl fmt::Display for UserAuthInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token {
            Some(token) => write!(
                f,
                "MediaBrowser \
                Client=\"{}\", \
                Device=\"{}\", \
                DeviceId=\"{}\", \
                Version=\"{}\", \
                Token=\"{}\"",
                self.client, self.device, self.device_id, self.version, token
            ),

            None => write!(
                f,
                "MediaBrowser \
                Client=\"{}\", \
                Device=\"{}\", \
                DeviceId=\"{}\", \
                Version=\"{}\"",
                self.client, self.device, self.device_id, self.version
            ),
        }
    }
}

// We want to be able to turn this back into a string
impl TryFrom<String> for UserAuthInfo {
    type Error = SeascapeError;

    fn try_from(mut user_auth_info_string: String) -> Result<Self, SeascapeError> {
        user_auth_info_string = user_auth_info_string.replace("MediaBrowser ", "");
        user_auth_info_string = user_auth_info_string.replace(r#"""#, "");

        let auth_info_pairs: HashMap<String, String> = user_auth_info_string
            .split(", ")
            .map(|kv| kv.split('='))
            .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into()))
            .collect::<HashMap<String, String>>();

        // TO DO: Look into better error handling here.
        Ok(UserAuthInfo {
            client: auth_info_pairs
                .get("Client")
                .ok_or(error::SeascapeError::Unknown)?
                .to_owned(),
            device: auth_info_pairs
                .get("Device")
                .ok_or(error::SeascapeError::Unknown)?
                .to_owned(),
            device_id: auth_info_pairs
                .get("DeviceId")
                .ok_or(error::SeascapeError::Unknown)?
                .to_owned(),
            version: auth_info_pairs
                .get("Version")
                .ok_or(error::SeascapeError::Unknown)?
                .to_owned(),
            token: auth_info_pairs.get("Token").map(String::from),
        })
    }
}
