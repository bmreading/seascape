use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectPlayProfile {
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::DlnaProfileType>>,
}

impl DirectPlayProfile {
    pub fn new() -> DirectPlayProfile {
        DirectPlayProfile {
            container: None,
            audio_codec: None,
            video_codec: None,
            _type: None,
        }
    }
}
