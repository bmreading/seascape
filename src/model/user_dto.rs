use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
    /// The name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The server identifier.
    #[serde(rename = "ServerId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// The name of the server.  This is not used by the server and is for client-side usage only.
    #[serde(rename = "ServerName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// The id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The primary image tag.
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option::is_none")]
    pub primary_image_tag: Option<String>,
    /// A value indicating whether this instance has password.
    #[serde(rename = "HasPassword", skip_serializing_if = "Option::is_none")]
    pub has_password: Option<bool>,
    /// A value indicating whether this instance has configured password.
    #[serde(
        rename = "HasConfiguredPassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_password: Option<bool>,
    /// A value indicating whether this instance has configured easy password.
    #[serde(
        rename = "HasConfiguredEasyPassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_configured_easy_password: Option<bool>,
    /// Whether async login is enabled or not.
    #[serde(rename = "EnableAutoLogin", skip_serializing_if = "Option::is_none")]
    pub enable_auto_login: Option<bool>,
    /// The last login date.
    #[serde(rename = "LastLoginDate", skip_serializing_if = "Option::is_none")]
    pub last_login_date: Option<String>,
    /// The last activity date.
    #[serde(rename = "LastActivityDate", skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<String>,
    /// The configuration.
    #[serde(rename = "Configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<crate::model::UserConfiguration>>,
    /// The policy.
    #[serde(rename = "Policy", skip_serializing_if = "Option::is_none")]
    pub policy: Option<Box<crate::model::UserPolicy>>,
    /// The primary image aspect ratio.
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
}