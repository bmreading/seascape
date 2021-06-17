use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaderInfo {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<Box<crate::model::HeaderMatchType>>,
}

impl HttpHeaderInfo {
    pub fn new() -> HttpHeaderInfo {
        HttpHeaderInfo {
            name: None,
            value: None,
            _match: None,
        }
    }
}
