use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerStateInfo {
    /// Gets or sets the now playing position ticks.
    #[serde(rename = "PositionTicks", skip_serializing_if = "Option::is_none")]
    pub position_ticks: Option<i64>,
    /// Gets or sets a value indicating whether this instance can seek.
    #[serde(rename = "CanSeek", skip_serializing_if = "Option::is_none")]
    pub can_seek: Option<bool>,
    /// Gets or sets a value indicating whether this instance is paused.
    #[serde(rename = "IsPaused", skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    /// Gets or sets a value indicating whether this instance is muted.
    #[serde(rename = "IsMuted", skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    /// Gets or sets the volume level.
    #[serde(rename = "VolumeLevel", skip_serializing_if = "Option::is_none")]
    pub volume_level: Option<i32>,
    /// Gets or sets the index of the now playing audio stream.
    #[serde(rename = "AudioStreamIndex", skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    /// Gets or sets the index of the now playing subtitle stream.
    #[serde(
        rename = "SubtitleStreamIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_stream_index: Option<i32>,
    /// Gets or sets the now playing media version identifier.
    #[serde(rename = "MediaSourceId", skip_serializing_if = "Option::is_none")]
    pub media_source_id: Option<String>,
    /// Gets or sets the play method.
    #[serde(rename = "PlayMethod", skip_serializing_if = "Option::is_none")]
    pub play_method: Option<Box<crate::model::PlayMethod>>,
    /// Gets or sets the repeat mode.
    #[serde(rename = "RepeatMode", skip_serializing_if = "Option::is_none")]
    pub repeat_mode: Option<Box<crate::model::RepeatMode>>,
}

impl PlayerStateInfo {
    pub fn new() -> PlayerStateInfo {
        PlayerStateInfo {
            position_ticks: None,
            can_seek: None,
            is_paused: None,
            is_muted: None,
            volume_level: None,
            audio_stream_index: None,
            subtitle_stream_index: None,
            media_source_id: None,
            play_method: None,
            repeat_mode: None,
        }
    }
}
