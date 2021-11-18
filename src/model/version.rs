use serde::{Deserialize, Serialize};

/// Represents a version number
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "Major", skip_serializing_if = "Option::is_none")]
    pub major: Option<i32>,
    #[serde(rename = "Minor", skip_serializing_if = "Option::is_none")]
    pub minor: Option<i32>,
    #[serde(rename = "Build", skip_serializing_if = "Option::is_none")]
    pub build: Option<i32>,
    #[serde(rename = "Revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "MajorRevision", skip_serializing_if = "Option::is_none")]
    pub major_revision: Option<i32>,
    #[serde(rename = "MinorRevision", skip_serializing_if = "Option::is_none")]
    pub minor_revision: Option<i32>,
}
