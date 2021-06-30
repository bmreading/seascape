use serde::{Deserialize, Serialize};

/// A media stream
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaStream {
    /// The codec.
    #[serde(rename = "Codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// The codec tag.
    #[serde(rename = "CodecTag", skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<String>,
    /// The language.
    #[serde(rename = "Language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The color range.
    #[serde(rename = "ColorRange", skip_serializing_if = "Option::is_none")]
    pub color_range: Option<String>,
    /// The color space.
    #[serde(rename = "ColorSpace", skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// The color transfer.
    #[serde(rename = "ColorTransfer", skip_serializing_if = "Option::is_none")]
    pub color_transfer: Option<String>,
    /// The color primaries.
    #[serde(rename = "ColorPrimaries", skip_serializing_if = "Option::is_none")]
    pub color_primaries: Option<String>,
    /// The comment.
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// The time base.
    #[serde(rename = "TimeBase", skip_serializing_if = "Option::is_none")]
    pub time_base: Option<String>,
    /// The codec time base.
    #[serde(rename = "CodecTimeBase", skip_serializing_if = "Option::is_none")]
    pub codec_time_base: Option<String>,
    /// The title.
    #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The video range.
    #[serde(rename = "VideoRange", skip_serializing_if = "Option::is_none")]
    pub video_range: Option<String>,
    #[serde(rename = "localizedUndefined", skip_serializing_if = "Option::is_none")]
    pub localized_undefined: Option<String>,
    #[serde(rename = "localizedDefault", skip_serializing_if = "Option::is_none")]
    pub localized_default: Option<String>,
    #[serde(rename = "localizedForced", skip_serializing_if = "Option::is_none")]
    pub localized_forced: Option<String>,
    #[serde(rename = "DisplayTitle", skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    #[serde(rename = "NalLengthSize", skip_serializing_if = "Option::is_none")]
    pub nal_length_size: Option<String>,
    /// A value indicating whether this instance is interlaced.
    #[serde(rename = "IsInterlaced", skip_serializing_if = "Option::is_none")]
    pub is_interlaced: Option<bool>,
    #[serde(rename = "IsAVC", skip_serializing_if = "Option::is_none")]
    pub is_avc: Option<bool>,
    /// The channel layout.
    #[serde(rename = "ChannelLayout", skip_serializing_if = "Option::is_none")]
    pub channel_layout: Option<String>,
    /// The bit rate.
    #[serde(rename = "BitRate", skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i32>,
    /// The bit depth.
    #[serde(rename = "BitDepth", skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i32>,
    /// The reference frames.
    #[serde(rename = "RefFrames", skip_serializing_if = "Option::is_none")]
    pub ref_frames: Option<i32>,
    /// The length of the packet.
    #[serde(rename = "PacketLength", skip_serializing_if = "Option::is_none")]
    pub packet_length: Option<i32>,
    /// The channels.
    #[serde(rename = "Channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    /// The sample rate.
    #[serde(rename = "SampleRate", skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    /// A value indicating whether this instance is default.
    #[serde(rename = "IsDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// A value indicating whether this instance is forced.
    #[serde(rename = "IsForced", skip_serializing_if = "Option::is_none")]
    pub is_forced: Option<bool>,
    /// The height.
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    /// The width.
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// The average frame rate.
    #[serde(rename = "AverageFrameRate", skip_serializing_if = "Option::is_none")]
    pub average_frame_rate: Option<f32>,
    /// The real frame rate.
    #[serde(rename = "RealFrameRate", skip_serializing_if = "Option::is_none")]
    pub real_frame_rate: Option<f32>,
    /// The profile.
    #[serde(rename = "Profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// The type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::MediaStreamType>>,
    /// The aspect ratio.
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The index.
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// The score.
    #[serde(rename = "Score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    /// A value indicating whether this instance is external.
    #[serde(rename = "IsExternal", skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    /// The method.
    #[serde(rename = "DeliveryMethod", skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<Box<crate::model::SubtitleDeliveryMethod>>,
    /// The delivery URL.
    #[serde(rename = "DeliveryUrl", skip_serializing_if = "Option::is_none")]
    pub delivery_url: Option<String>,
    /// A value indicating whether this instance is external URL.
    #[serde(rename = "IsExternalUrl", skip_serializing_if = "Option::is_none")]
    pub is_external_url: Option<bool>,
    #[serde(
        rename = "IsTextSubtitleStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_text_subtitle_stream: Option<bool>,
    /// A value indicating whether [supports external stream].
    #[serde(
        rename = "SupportsExternalStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_external_stream: Option<bool>,
    /// The filename.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The pixel format.
    #[serde(rename = "PixelFormat", skip_serializing_if = "Option::is_none")]
    pub pixel_format: Option<String>,
    /// The level.
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    /// Gets a value indicating whether this instance is anamorphic.
    #[serde(rename = "IsAnamorphic", skip_serializing_if = "Option::is_none")]
    pub is_anamorphic: Option<bool>,
}

impl MediaStream {
    /// Class MediaStream.
    pub fn new() -> MediaStream {
        MediaStream {
            codec: None,
            codec_tag: None,
            language: None,
            color_range: None,
            color_space: None,
            color_transfer: None,
            color_primaries: None,
            comment: None,
            time_base: None,
            codec_time_base: None,
            title: None,
            video_range: None,
            localized_undefined: None,
            localized_default: None,
            localized_forced: None,
            display_title: None,
            nal_length_size: None,
            is_interlaced: None,
            is_avc: None,
            channel_layout: None,
            bit_rate: None,
            bit_depth: None,
            ref_frames: None,
            packet_length: None,
            channels: None,
            sample_rate: None,
            is_default: None,
            is_forced: None,
            height: None,
            width: None,
            average_frame_rate: None,
            real_frame_rate: None,
            profile: None,
            _type: None,
            aspect_ratio: None,
            index: None,
            score: None,
            is_external: None,
            delivery_method: None,
            delivery_url: None,
            is_external_url: None,
            is_text_subtitle_stream: None,
            supports_external_stream: None,
            path: None,
            pixel_format: None,
            level: None,
            is_anamorphic: None,
        }
    }
}
