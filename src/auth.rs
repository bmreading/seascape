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
    token: Option<String>,
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

    pub fn set_token(&mut self, token: &str) -> &UserAuthInfo {
        self.token = Some(String::from(token));
        self
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

// We want to be able to deserialize this back into an object
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_auth_info_inits_client() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );
        assert_eq!(user_auth_info.client, "Fake Client");
    }

    #[test]
    fn new_user_auth_info_inits_device() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );
        assert_eq!(user_auth_info.device, "Fake Device");
    }

    #[test]
    fn new_user_auth_info_inits_device_id() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );
        assert_eq!(user_auth_info.device_id, "Fake ID");
    }

    #[test]
    fn new_user_auth_info_inits_version() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );
        assert_eq!(user_auth_info.version, "0.1.0");
    }

    #[test]
    fn new_user_auth_info_inits_some_token() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );
        assert_eq!(user_auth_info.token.is_some(), true);
    }

    #[test]
    fn new_user_auth_info_inits_none_token() {
        let user_auth_info =
            UserAuthInfo::new("Fake Client", "Fake Device", "Fake ID", "0.1.0", None);
        assert_eq!(user_auth_info.token.is_none(), true);
    }

    #[test]
    fn user_auth_info_serializes() {
        let user_auth_info = UserAuthInfo::new(
            "Fake Client",
            "Fake Device",
            "Fake ID",
            "0.1.0",
            Some("Fake Token"),
        );

        let user_auth_info_string = user_auth_info.to_string();

        assert_eq!(
            user_auth_info_string.as_str(),
            "MediaBrowser Client=\"Fake Client\", \
            Device=\"Fake Device\", \
            DeviceId=\"Fake ID\", \
            Version=\"0.1.0\", \
            Token=\"Fake Token\""
        );
    }

    #[test]
    fn user_auth_info_deserializes() {
        let user_auth_info_string = String::from(
            "MediaBrowser Client=\"Fake Client\", \
            Device=\"Fake Device\", \
            DeviceId=\"Fake ID\", \
            Version=\"0.1.0\", \
            Token=\"Fake Token\"",
        );

        let user_auth_info = UserAuthInfo::try_from(user_auth_info_string);

        assert_eq!(user_auth_info.is_ok(), true);
        assert_eq!(user_auth_info.as_ref().unwrap().client, "Fake Client");
        assert_eq!(user_auth_info.as_ref().unwrap().device, "Fake Device");
        assert_eq!(user_auth_info.as_ref().unwrap().device_id, "Fake ID");
        assert_eq!(user_auth_info.as_ref().unwrap().version, "0.1.0");
        assert_eq!(user_auth_info.as_ref().unwrap().token.is_some(), true);
    }

    #[test]
    fn user_auth_info_sets_token() {
        let mut user_auth_info =
            UserAuthInfo::new("Fake Client", "Fake Device", "Fake ID", "0.1.0", None);

        user_auth_info.set_token("Fake Token");

        assert_eq!(user_auth_info.token.unwrap(), String::from("Fake Token"));
    }
}
