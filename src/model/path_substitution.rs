use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PathSubstitution {
    /// Gets or sets the value to substitute.
    #[serde(rename = "From", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Gets or sets the value to substitution with.
    #[serde(rename = "To", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

