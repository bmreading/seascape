use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaUrl {
    #[serde(rename = "Url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MediaUrl {
    pub fn new() -> MediaUrl {
        MediaUrl {
            url: None,
            name: None,
        }
    }
}
