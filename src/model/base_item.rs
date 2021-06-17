use serde::{Deserialize, Serialize};

/// BaseItem : Class BaseItem.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItem {
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "DateLastSaved", skip_serializing_if = "Option::is_none")]
    pub date_last_saved: Option<String>,
    /// Gets or sets the remote trailers.
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option::is_none")]
    pub remote_trailers: Option<Vec<crate::model::MediaUrl>>,
    #[serde(rename = "IsHD", skip_serializing_if = "Option::is_none")]
    pub is_hd: Option<bool>,
    #[serde(rename = "IsShortcut", skip_serializing_if = "Option::is_none")]
    pub is_shortcut: Option<bool>,
    #[serde(rename = "ShortcutPath", skip_serializing_if = "Option::is_none")]
    pub shortcut_path: Option<String>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "ExtraIds", skip_serializing_if = "Option::is_none")]
    pub extra_ids: Option<Vec<String>>,
    #[serde(
        rename = "SupportsExternalTransfer",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_external_transfer: Option<bool>,
}

impl BaseItem {
    /// Class BaseItem.
    pub fn new() -> BaseItem {
        BaseItem {
            size: None,
            container: None,
            date_last_saved: None,
            remote_trailers: None,
            is_hd: None,
            is_shortcut: None,
            shortcut_path: None,
            width: None,
            height: None,
            extra_ids: None,
            supports_external_transfer: None,
        }
    }
}
