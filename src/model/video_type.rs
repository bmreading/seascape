use serde::{Deserialize, Serialize};

/// VideoType : Enum VideoType.

/// Enum VideoType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VideoType {
    #[serde(rename = "VideoFile")]
    VideoFile,
    #[serde(rename = "Iso")]
    Iso,
    #[serde(rename = "Dvd")]
    Dvd,
    #[serde(rename = "BluRay")]
    BluRay,
}

impl ToString for VideoType {
    fn to_string(&self) -> String {
        match self {
            Self::VideoFile => String::from("VideoFile"),
            Self::Iso => String::from("Iso"),
            Self::Dvd => String::from("Dvd"),
            Self::BluRay => String::from("BluRay"),
        }
    }
}
