use serde::{Deserialize, Serialize};

/// ChapterInfo : Class ChapterInfo.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterInfo {
    /// Gets or sets the start position ticks.
    #[serde(rename = "StartPositionTicks", skip_serializing_if = "Option::is_none")]
    pub start_position_ticks: Option<i64>,
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the image path.
    #[serde(rename = "ImagePath", skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(rename = "ImageDateModified", skip_serializing_if = "Option::is_none")]
    pub image_date_modified: Option<String>,
    #[serde(rename = "ImageTag", skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
}

impl ChapterInfo {
    /// Class ChapterInfo.
    pub fn new() -> ChapterInfo {
        ChapterInfo {
            start_position_ticks: None,
            name: None,
            image_path: None,
            image_date_modified: None,
            image_tag: None,
        }
    }
}
