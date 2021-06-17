use serde::{Deserialize, Serialize};

/// XmlAttribute : Defines the MediaBrowser.Model.Dlna.XmlAttribute.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XmlAttribute {
    /// Gets or sets the name of the attribute.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the value of the attribute.
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl XmlAttribute {
    /// Defines the MediaBrowser.Model.Dlna.XmlAttribute.
    pub fn new() -> XmlAttribute {
        XmlAttribute {
            name: None,
            value: None,
        }
    }
}
