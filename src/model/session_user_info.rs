use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUserInfo {
    /// Gets or sets the user identifier.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Gets or sets the name of the user.
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl SessionUserInfo {
    /// Class SessionUserInfo.
    pub fn new() -> SessionUserInfo {
        SessionUserInfo {
            user_id: None,
            user_name: None,
        }
    }
}
