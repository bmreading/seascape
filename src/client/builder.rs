//! Build Jellyfin clients

use crate::auth::{ApiKeyAuthHeader, AuthHeaderType, UserAuthHeader, UserAuthInfo};
use crate::error::SeascapeError;
use crate::http::{AsyncClient, HttpClientType};

use super::{ClientResult, Jellyfin};

/// A builder used to construct a Jellyfin client, which serves as the main
/// entrypoint for the library.
#[derive(Debug)]
pub struct Builder {
    http_client_type: HttpClientType,
    base_url: String,
    username: Option<String>,
    password: Option<String>,
    api_key: Option<String>,
    client: Option<String>,
    device: Option<String>,
    device_id: Option<String>,
    version: Option<String>,
    token: Option<String>,
    auth_header_type: Option<AuthHeaderType>,
}

impl Builder {
    /// Returns a new base client builder with only its base URL field filled
    pub fn new(base_url: &str) -> Self {
        let async_client = AsyncClient::default();
        let http_client_type = HttpClientType::AsyncClient(async_client);

        Self {
            base_url: base_url.to_string(),
            http_client_type,
            username: None,
            password: None,
            api_key: None,
            client: None,
            device: None,
            device_id: None,
            version: None,
            token: None,
            auth_header_type: None,
        }
    }

    /// Sets an HTTP client type (with an actual HTTP client by design)
    pub fn http_client_type(&mut self, http_client_type: HttpClientType) -> &mut Self {
        self.http_client_type = http_client_type;
        self
    }

    /// Sets a username.
    /// Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_string());
        self
    }

    /// Sets a password.
    /// Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    /// Sets an API key.
    /// Required for API key authentication. If this key is
    /// set, then clientbuilder will return an authenticated
    /// client upon being consumed. Do not use if performing
    /// user-based authentication.
    pub fn api_key(&mut self, api_key: &str) -> &mut Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    /// Sets the type of the client that will be used such as
    /// "Android", "PC", etc. Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn client(&mut self, client: &str) -> &mut Self {
        self.client = Some(client.to_string());
        self
    }

    /// Sets the product name of the device that will be used such as "Samsung Galaxy SIII".
    /// Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn device(&mut self, device: &str) -> &mut Self {
        self.device = Some(device.to_string());
        self
    }

    /// Sets a unique identifier for the device that will be used.
    /// Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn device_id(&mut self, device_id: &str) -> &mut Self {
        self.device_id = Some(device_id.to_string());
        self
    }

    /// Sets the version number of the client application that will be used.
    /// Required for user-based authentication.
    /// Do not use if performing API key-based authentication.
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = Some(version.to_string());
        self
    }

    /// Sets an access token for the session to be used.
    /// This does not usually need to be set. If all required
    /// fields are set for a user-based authentication, then
    /// an authenticated client is automatically returned.
    /// Do not use if performing API key-based authentication.
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }

    /// Sets the authentication header type (user or api key-based).
    /// If either all other user-based authentication detail fields
    /// or the API key field is set, this will be automatically
    /// generated. This is only useful to set if you have created
    /// an authentication header manually.
    pub fn auth_header_type(&mut self, auth_header_type: AuthHeaderType) -> &mut Self {
        self.auth_header_type = Some(auth_header_type);
        self
    }

    /// Consumes the builder and returns a client
    pub async fn build(&self) -> ClientResult<Jellyfin> {
        // Can't do api key and user auth simultaneously
        if (self.api_key.is_some() && self.username.is_some())
            || (self.api_key.is_some() && self.password.is_some())
        {
            Err(SeascapeError::InvalidClient(
                "cannot use api key auth and user auth simultaneously".to_string(),
            ))

        // Can't have a password without a username or vice-versa
        } else if (self.username.is_some() && self.password.is_none())
            || (self.password.is_some() && self.username.is_none())
        {
            Err(SeascapeError::InvalidClient(
                "username and password must be used together".to_string(),
            ))

        // Can't have username nor password without user auth params
        } else if (self.username.is_some() && self.password.is_some()) && (self.client.is_none())
            || (self.username.is_some() && self.password.is_some()) && (self.device.is_none())
            || (self.username.is_some() && self.password.is_some()) && (self.device_id.is_none())
            || (self.username.is_some() && self.password.is_some()) && (self.version.is_none())
        {
            Err(SeascapeError::InvalidClient(
                "user auth requires credentials and user auth params".to_string(),
            ))

        // Can't have both api key and any user auth params
        } else if (self.api_key.is_some() && self.client.is_some())
            || (self.api_key.is_some() && self.device.is_some())
            || (self.api_key.is_some() && self.device_id.is_some())
            || (self.api_key.is_some() && self.version.is_some())
            || (self.api_key.is_some() && self.token.is_some())
        {
            Err(SeascapeError::InvalidClient(
                "client, device, device_id, version, and token are only valid with user auth"
                    .to_string(),
            ))

        // Can't have token and user/pass creds
        } else if (self.token.is_some() && self.username.is_some())
            || (self.token.is_some() && self.password.is_some())
        {
            Err(SeascapeError::InvalidClient(
                "token cannot be used with username and password credentials".to_string(),
            ))

        // Can't use auth_header with any of these
        } else if (self.auth_header_type.is_some() && self.username.is_some())
            || (self.auth_header_type.is_some() && self.password.is_some())
            || (self.auth_header_type.is_some() && self.client.is_some())
            || (self.auth_header_type.is_some() && self.device.is_some())
            || (self.auth_header_type.is_some() && self.device_id.is_some())
            || (self.auth_header_type.is_some() && self.version.is_some())
            || (self.auth_header_type.is_some() && self.token.is_some())
            || (self.auth_header_type.is_some() && self.api_key.is_some())
        {
            Err(SeascapeError::InvalidClient(
                "auth_header cannot be used with username, password, client, device, device_id, version, nor token"
                    .to_string(),
            ))

        // Try a user authentication
        } else if self.username.is_some()
            && self.password.is_some()
            && self.client.is_some()
            && self.device.is_some()
            && self.device_id.is_some()
            && self.version.is_some()
            && self.token.is_none()
            && self.api_key.is_none()
        {
            let user_auth_info = UserAuthInfo::new(
                self.client.as_ref().unwrap(),
                &self.device.as_ref().unwrap(),
                &self.device_id.as_ref().unwrap(),
                &self.version.as_ref().unwrap(),
                None,
            );
            let user_header = UserAuthHeader::new(user_auth_info);
            let auth_header_type = AuthHeaderType::UserAuthHeaderType(user_header);

            let jellyfin = Jellyfin {
                http_client_type: self.http_client_type.to_owned(),
                auth_header_type: Some(auth_header_type),
                auth_result: None,
                base_url: self.base_url.to_string(),
            };

            Ok(jellyfin
                .authenticate_by_user(
                    &self.username.as_ref().unwrap(),
                    &self.password.as_ref().unwrap(),
                )
                .await?)
        // Return with api key authentication
        } else if self.api_key.is_some()
            && self.username.is_none()
            && self.password.is_none()
            && self.client.is_none()
            && self.device.is_none()
            && self.device_id.is_none()
            && self.version.is_none()
            && self.token.is_none()
        {
            let api_key_header = ApiKeyAuthHeader::new(&self.api_key.as_ref().unwrap());
            let auth_header_type = AuthHeaderType::ApiKeyAuthHeaderType(api_key_header);

            Ok(Jellyfin {
                http_client_type: self.http_client_type.to_owned(),
                auth_header_type: Some(auth_header_type),
                auth_result: None,
                base_url: self.base_url.to_owned(),
            })
        } else {
            Ok(Jellyfin {
                http_client_type: self.http_client_type.to_owned(),
                auth_header_type: None,
                auth_result: None,
                base_url: self.base_url.to_owned(),
            })
        }
    }
}
