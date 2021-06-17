//! Items specific to an Authentication Result
use serde::{Deserialize, Serialize};

/// Represents an authentication result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// A user data transfer object.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::model::UserDto>>,
    /// Session information.
    #[serde(rename = "SessionInfo", skip_serializing_if = "Option::is_none")]
    pub session_info: Option<Box<crate::model::SessionInfo>>,
    /// An access token to be used for API calls needing authentication.
    #[serde(rename = "AccessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// A unique identifier of the server that has been authenticated to.
    #[serde(rename = "ServerId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
}

impl AuthenticationResult {
    /// Returns a new authentication result.
    pub fn new() -> AuthenticationResult {
        AuthenticationResult {
            user: None,
            session_info: None,
            access_token: None,
            server_id: None,
        }
    }
}
