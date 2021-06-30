use serde::{Deserialize, Serialize};

/// Stores information about a Person within a BaseItem.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemPerson {
    /// The name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The identifier.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The role.
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The primary image tag.
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option::is_none")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Box<crate::model::BaseItemPersonImageBlurHashes>>,
}