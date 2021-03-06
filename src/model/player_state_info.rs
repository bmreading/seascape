use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerStateInfo {
    /// The now playing position ticks.
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    /// A value indicating whether this instance can seek.
    #[serde(rename = "CanSeek", skip_serializing_if = "Option::is_none")]
    pub can_seek: Option<bool>,
    /// A value indicating whether this instance is paused.
    #[serde(rename = "IsPaused", skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// A value indicating whether this instance is muted.
    #[serde(rename = "IsMuted", skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    /// The volume level.
    #[serde(rename = "VolumeLevel", skip_serializing_if = "Option::is_none")]
    pub volume_level: Option<i32>,
    /// The index of the now playing audio stream.
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    /// The index of the now playing subtitle stream.
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
    /// The now playing media version identifier.
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option::is_none")]
    pub media_source_id: Option<String>,
    /// The play method.
    #[serde(rename = "PlayMethod", skip_serializing_if = "Option::is_none")]
    pub play_method: Option<Box<crate::model::PlayMethod>>,
    /// The repeat mode.
    #[serde(rename = "RepeatMode", skip_serializing_if = "Option::is_none")]
    pub repeat_mode: Option<Box<crate::model::RepeatMode>>,
}
