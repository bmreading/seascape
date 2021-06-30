use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MediaSourceInfo {
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Box<crate::model::MediaProtocol>>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "EncoderPath", skip_serializing_if = "Option::is_none")]
    pub encoder_path: Option<String>,
    #[serde(rename = "EncoderProtocol", skip_serializing_if = "Option::is_none")]
    pub encoder_protocol: Option<Box<crate::model::MediaProtocol>>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::MediaSourceType>>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Differentiate internet url vs local network.
    #[serde(rename = "IsRemote", skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<bool>,
    #[serde(rename = "ETag", skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(
        rename = "ReadAtNativeFramerate",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_at_native_framerate: Option<bool>,
    #[serde(rename = "IgnoreDts", skip_serializing_if = "Option::is_none")]
    pub ignore_dts: Option<bool>,
    #[serde(rename = "IgnoreIndex", skip_serializing_if = "Option::is_none")]
    pub ignore_index: Option<bool>,
    #[serde(rename = "GenPtsInput", skip_serializing_if = "Option::is_none")]
    pub gen_pts_input: Option<bool>,
    #[serde(
        rename = "SupportsTranscoding",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_transcoding: Option<bool>,
    #[serde(
        rename = "SupportsDirectStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_direct_stream: Option<bool>,
    #[serde(rename = "SupportsDirectPlay", skip_serializing_if = "Option::is_none")]
    pub supports_direct_play: Option<bool>,
    #[serde(rename = "IsInfiniteStream", skip_serializing_if = "Option::is_none")]
    pub is_infinite_stream: Option<bool>,
    #[serde(rename = "RequiresOpening", skip_serializing_if = "Option::is_none")]
    pub requires_opening: Option<bool>,
    #[serde(rename = "OpenToken", skip_serializing_if = "Option::is_none")]
    pub open_token: Option<String>,
    #[serde(rename = "RequiresClosing", skip_serializing_if = "Option::is_none")]
    pub requires_closing: Option<bool>,
    #[serde(rename = "LiveStreamId", skip_serializing_if = "Option::is_none")]
    pub live_stream_id: Option<String>,
    #[serde(rename = "BufferMs", skip_serializing_if = "Option::is_none")]
    pub buffer_ms: Option<i32>,
    #[serde(rename = "RequiresLooping", skip_serializing_if = "Option::is_none")]
    pub requires_looping: Option<bool>,
    #[serde(rename = "SupportsProbing", skip_serializing_if = "Option::is_none")]
    pub supports_probing: Option<bool>,
    #[serde(rename = "VideoType", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<Box<crate::model::VideoType>>,
    #[serde(rename = "IsoType", skip_serializing_if = "Option::is_none")]
    pub iso_type: Option<Box<crate::model::IsoType>>,
    #[serde(rename = "Video3DFormat", skip_serializing_if = "Option::is_none")]
    pub video3_d_format: Option<Box<crate::model::Video3DFormat>>,
    #[serde(rename = "MediaStreams", skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<crate::model::MediaStream>>,
    #[serde(rename = "MediaAttachments", skip_serializing_if = "Option::is_none")]
    pub media_attachments: Option<Vec<crate::model::MediaAttachment>>,
    #[serde(rename = "Formats", skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
    #[serde(rename = "Bitrate", skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Box<crate::model::TransportStreamTimestamp>>,
    #[serde(
        rename = "RequiredHttpHeaders",
        skip_serializing_if = "Option::is_none"
    )]
    pub required_http_headers: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "TranscodingUrl", skip_serializing_if = "Option::is_none")]
    pub transcoding_url: Option<String>,
    #[serde(
        rename = "TranscodingSubProtocol",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_sub_protocol: Option<String>,
    #[serde(
        rename = "TranscodingContainer",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_container: Option<String>,
    #[serde(rename = "AnalyzeDurationMs", skip_serializing_if = "Option::is_none")]
    pub analyze_duration_ms: Option<i32>,
    #[serde(
        rename = "DefaultAudioStreamIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_audio_stream_index: Option<i32>,
    #[serde(
        rename = "DefaultSubtitleStreamIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_subtitle_stream_index: Option<i32>,
}