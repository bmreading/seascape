use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscodingInfo {
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "IsVideoDirect", skip_serializing_if = "Option::is_none")]
    pub is_video_direct: Option<bool>,
    #[serde(rename = "IsAudioDirect", skip_serializing_if = "Option::is_none")]
    pub is_audio_direct: Option<bool>,
    #[serde(rename = "Bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "Framerate", skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f32>,
    #[serde(
        rename = "CompletionPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_percentage: Option<f64>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "AudioChannels", skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i32>,
    #[serde(rename = "TranscodeReasons", skip_serializing_if = "Option::is_none")]
    pub transcode_reasons: Option<Vec<crate::model::TranscodeReason>>,
}

impl TranscodingInfo {
    pub fn new() -> TranscodingInfo {
        TranscodingInfo {
            audio_codec: None,
            video_codec: None,
            container: None,
            is_video_direct: None,
            is_audio_direct: None,
            bitrate: None,
            framerate: None,
            completion_percentage: None,
            width: None,
            height: None,
            audio_channels: None,
            transcode_reasons: None,
        }
    }
}
