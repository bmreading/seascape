use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameGuidPair {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl NameGuidPair {
    pub fn new() -> NameGuidPair {
        NameGuidPair {
            name: None,
            id: None,
        }
    }
}
