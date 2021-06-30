use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SessionUserInfo {
    /// The user identifier.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The name of the user.
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
