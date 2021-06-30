use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TranscodingProfile {
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::DlnaProfileType>>,
    #[serde(rename = "VideoCodec", skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    #[serde(rename = "AudioCodec", skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<String>,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(
        rename = "EstimateContentLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub estimate_content_length: Option<bool>,
    #[serde(
        rename = "EnableMpegtsM2TsMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_mpegts_m2_ts_mode: Option<bool>,
    #[serde(rename = "TranscodeSeekInfo", skip_serializing_if = "Option::is_none")]
    pub transcode_seek_info: Option<Box<crate::model::TranscodeSeekInfo>>,
    #[serde(rename = "CopyTimestamps", skip_serializing_if = "Option::is_none")]
    pub copy_timestamps: Option<bool>,
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<crate::model::EncodingContext>>,
    #[serde(
        rename = "EnableSubtitlesInManifest",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_subtitles_in_manifest: Option<bool>,
    #[serde(rename = "MaxAudioChannels", skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<String>,
    #[serde(rename = "MinSegments", skip_serializing_if = "Option::is_none")]
    pub min_segments: Option<i32>,
    #[serde(rename = "SegmentLength", skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i32>,
    #[serde(
        rename = "BreakOnNonKeyFrames",
        skip_serializing_if = "Option::is_none"
    )]
    pub break_on_non_key_frames: Option<bool>,
}
