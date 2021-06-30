use serde::{Deserialize, Serialize};

/// A DLNA DeviceProfile.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    /// The Name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The Identification.
    #[serde(rename = "Identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Box<crate::model::DeviceIdentification>>,
    /// The FriendlyName.
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The Manufacturer.
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The Manufacturer Url.
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option::is_none")]
    pub manufacturer_url: Option<String>,
    /// The Model Name.
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The Model Description.
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    /// The Model Number.
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
    /// The Model Url.
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option::is_none")]
    pub model_url: Option<String>,
    /// The Serial Number.
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// A value indicating whether Album Art is enabled In Didl.
    #[serde(
        rename = "EnableAlbumArtInDidl",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_album_art_in_didl: Option<bool>,
    /// A value indicating whether Single Album Art Limit is enabled.
    #[serde(
        rename = "EnableSingleAlbumArtLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_album_art_limit: Option<bool>,
    /// A value indicating whether Single Subtitle Limit is enabled.
    #[serde(
        rename = "EnableSingleSubtitleLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_subtitle_limit: Option<bool>,
    /// The supported media types.
    #[serde(
        rename = "SupportedMediaTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_media_types: Option<String>,
    /// The User Id.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The Album Art Pn.
    #[serde(rename = "AlbumArtPn", skip_serializing_if = "Option::is_none")]
    pub album_art_pn: Option<String>,
    /// The maximum album art width.
    #[serde(rename = "MaxAlbumArtWidth", skip_serializing_if = "Option::is_none")]
    pub max_album_art_width: Option<i32>,
    /// The maximum album art height.
    #[serde(rename = "MaxAlbumArtHeight", skip_serializing_if = "Option::is_none")]
    pub max_album_art_height: Option<i32>,
    /// The maximum icon width.
    #[serde(rename = "MaxIconWidth", skip_serializing_if = "Option::is_none")]
    pub max_icon_width: Option<i32>,
    /// The maximum icon height.
    #[serde(rename = "MaxIconHeight", skip_serializing_if = "Option::is_none")]
    pub max_icon_height: Option<i32>,
    /// The maximum streaming bitrate.
    #[serde(
        rename = "MaxStreamingBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<i32>,
    /// The maximum static bitrate.
    #[serde(rename = "MaxStaticBitrate", skip_serializing_if = "Option::is_none")]
    pub max_static_bitrate: Option<i32>,
    /// The music streaming transcoding bitrate.
    #[serde(
        rename = "MusicStreamingTranscodingBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub music_streaming_transcoding_bitrate: Option<i32>,
    /// The maximum static music bitrate.
    #[serde(
        rename = "MaxStaticMusicBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_music_bitrate: Option<i32>,
    /// The content of the aggregationFlags element in the urn:schemas-sonycom:av namespace.
    #[serde(
        rename = "SonyAggregationFlags",
        skip_serializing_if = "Option::is_none"
    )]
    pub sony_aggregation_flags: Option<String>,
    /// The Protocol Info.
    #[serde(rename = "ProtocolInfo", skip_serializing_if = "Option::is_none")]
    pub protocol_info: Option<String>,
    /// The Timeline Offset Seconds.
    #[serde(
        rename = "TimelineOffsetSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_offset_seconds: Option<i32>,
    /// A value indicating whether plain video items are required.
    #[serde(
        rename = "RequiresPlainVideoItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_video_items: Option<bool>,
    /// A value indicating whether plain folders are required.
    #[serde(
        rename = "RequiresPlainFolders",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_folders: Option<bool>,
    /// A value indicating whether MSMediaReceiverRegistrar is enabled.
    #[serde(
        rename = "EnableMSMediaReceiverRegistrar",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ms_media_receiver_registrar: Option<bool>,
    /// A value indicating whether to ignore transcode byte range requests.
    #[serde(
        rename = "IgnoreTranscodeByteRangeRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_transcode_byte_range_requests: Option<bool>,
    /// The XML root attributes.
    #[serde(rename = "XmlRootAttributes", skip_serializing_if = "Option::is_none")]
    pub xml_root_attributes: Option<Vec<crate::model::XmlAttribute>>,
    /// The direct play profiles.
    #[serde(rename = "DirectPlayProfiles", skip_serializing_if = "Option::is_none")]
    pub direct_play_profiles: Option<Vec<crate::model::DirectPlayProfile>>,
    /// The transcoding profiles.
    #[serde(
        rename = "TranscodingProfiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_profiles: Option<Vec<crate::model::TranscodingProfile>>,
    /// The container profiles.
    #[serde(rename = "ContainerProfiles", skip_serializing_if = "Option::is_none")]
    pub container_profiles: Option<Vec<crate::model::ContainerProfile>>,
    /// The codec profiles.
    #[serde(rename = "CodecProfiles", skip_serializing_if = "Option::is_none")]
    pub codec_profiles: Option<Vec<crate::model::CodecProfile>>,
    /// The response profiles.
    #[serde(rename = "ResponseProfiles", skip_serializing_if = "Option::is_none")]
    pub response_profiles: Option<Vec<crate::model::ResponseProfile>>,
    /// The subtitle profiles.
    #[serde(rename = "SubtitleProfiles", skip_serializing_if = "Option::is_none")]
    pub subtitle_profiles: Option<Vec<crate::model::SubtitleProfile>>,
}