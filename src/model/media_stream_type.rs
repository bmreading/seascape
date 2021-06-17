use serde::{Deserialize, Serialize};

/// MediaStreamType : Enum MediaStreamType.

/// Enum MediaStreamType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaStreamType {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Subtitle")]
    Subtitle,
    #[serde(rename = "EmbeddedImage")]
    EmbeddedImage,
}

impl ToString for MediaStreamType {
    fn to_string(&self) -> String {
        match self {
            Self::Audio => String::from("Audio"),
            Self::Video => String::from("Video"),
            Self::Subtitle => String::from("Subtitle"),
            Self::EmbeddedImage => String::from("EmbeddedImage"),
        }
    }
}
