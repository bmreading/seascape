use serde::{Deserialize, Serialize};

/// A DLNA XML attribute.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XmlAttribute {
    /// The name of the attribute.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the attribute.
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
