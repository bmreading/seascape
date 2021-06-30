use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExternalUrl {
    /// The name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the item.
    #[serde(rename = "Url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}