use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DlnaProfileType {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Photo")]
    Photo,
}

impl ToString for DlnaProfileType {
    fn to_string(&self) -> String {
        match self {
            Self::Audio => String::from("Audio"),
            Self::Video => String::from("Video"),
            Self::Photo => String::from("Photo"),
        }
    }
}
