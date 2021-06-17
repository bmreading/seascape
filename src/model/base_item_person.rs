use serde::{Deserialize, Serialize};

/// BaseItemPerson : This is used by the api to get information about a Person within a BaseItem.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemPerson {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the identifier.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets or sets the role.
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Gets or sets the type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Gets or sets the primary image tag.
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option::is_none")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Box<crate::model::BaseItemPersonImageBlurHashes>>,
}

impl BaseItemPerson {
    /// This is used by the api to get information about a Person within a BaseItem.
    pub fn new() -> BaseItemPerson {
        BaseItemPerson {
            name: None,
            id: None,
            role: None,
            _type: None,
            primary_image_tag: None,
            image_blur_hashes: None,
        }
    }
}
