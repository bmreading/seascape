use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodecType {
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "VideoAudio")]
    VideoAudio,
    #[serde(rename = "Audio")]
    Audio,
}

impl ToString for CodecType {
    fn to_string(&self) -> String {
        match self {
            Self::Video => String::from("Video"),
            Self::VideoAudio => String::from("VideoAudio"),
            Self::Audio => String::from("Audio"),
        }
    }
}
