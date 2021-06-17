use serde::{Deserialize, Serialize};
/// SubtitlePlaybackMode : An enum representing a subtitle playback mode.

/// An enum representing a subtitle playback mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubtitlePlaybackMode {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Always")]
    Always,
    #[serde(rename = "OnlyForced")]
    OnlyForced,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Smart")]
    Smart,
}

impl ToString for SubtitlePlaybackMode {
    fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("Default"),
            Self::Always => String::from("Always"),
            Self::OnlyForced => String::from("OnlyForced"),
            Self::None => String::from("None"),
            Self::Smart => String::from("Smart"),
        }
    }
}
