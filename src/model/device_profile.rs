use serde::{Deserialize, Serialize};
/// DeviceProfile : Defines the MediaBrowser.Model.Dlna.DeviceProfile.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    /// Gets or sets the Name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Gets or sets the Id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets or sets the Identification.
    #[serde(rename = "Identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Box<crate::model::DeviceIdentification>>,
    /// Gets or sets the FriendlyName.
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Gets or sets the Manufacturer.
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Gets or sets the ManufacturerUrl.
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option::is_none")]
    pub manufacturer_url: Option<String>,
    /// Gets or sets the ModelName.
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// Gets or sets the ModelDescription.
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    /// Gets or sets the ModelNumber.
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
    /// Gets or sets the ModelUrl.
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option::is_none")]
    pub model_url: Option<String>,
    /// Gets or sets the SerialNumber.
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Gets or sets a value indicating whether EnableAlbumArtInDidl.
    #[serde(
        rename = "EnableAlbumArtInDidl",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_album_art_in_didl: Option<bool>,
    /// Gets or sets a value indicating whether EnableSingleAlbumArtLimit.
    #[serde(
        rename = "EnableSingleAlbumArtLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_album_art_limit: Option<bool>,
    /// Gets or sets a value indicating whether EnableSingleSubtitleLimit.
    #[serde(
        rename = "EnableSingleSubtitleLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_single_subtitle_limit: Option<bool>,
    /// Gets or sets the SupportedMediaTypes.
    #[serde(
        rename = "SupportedMediaTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub supported_media_types: Option<String>,
    /// Gets or sets the UserId.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Gets or sets the AlbumArtPn.
    #[serde(rename = "AlbumArtPn", skip_serializing_if = "Option::is_none")]
    pub album_art_pn: Option<String>,
    /// Gets or sets the MaxAlbumArtWidth.
    #[serde(rename = "MaxAlbumArtWidth", skip_serializing_if = "Option::is_none")]
    pub max_album_art_width: Option<i32>,
    /// Gets or sets the MaxAlbumArtHeight.
    #[serde(rename = "MaxAlbumArtHeight", skip_serializing_if = "Option::is_none")]
    pub max_album_art_height: Option<i32>,
    /// Gets or sets the MaxIconWidth.
    #[serde(rename = "MaxIconWidth", skip_serializing_if = "Option::is_none")]
    pub max_icon_width: Option<i32>,
    /// Gets or sets the MaxIconHeight.
    #[serde(rename = "MaxIconHeight", skip_serializing_if = "Option::is_none")]
    pub max_icon_height: Option<i32>,
    /// Gets or sets the MaxStreamingBitrate.
    #[serde(
        rename = "MaxStreamingBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_streaming_bitrate: Option<i32>,
    /// Gets or sets the MaxStaticBitrate.
    #[serde(rename = "MaxStaticBitrate", skip_serializing_if = "Option::is_none")]
    pub max_static_bitrate: Option<i32>,
    /// Gets or sets the MusicStreamingTranscodingBitrate.
    #[serde(
        rename = "MusicStreamingTranscodingBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub music_streaming_transcoding_bitrate: Option<i32>,
    /// Gets or sets the MaxStaticMusicBitrate.
    #[serde(
        rename = "MaxStaticMusicBitrate",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_static_music_bitrate: Option<i32>,
    /// Gets or sets the content of the aggregationFlags element in the urn:schemas-sonycom:av namespace.
    #[serde(
        rename = "SonyAggregationFlags",
        skip_serializing_if = "Option::is_none"
    )]
    pub sony_aggregation_flags: Option<String>,
    /// Gets or sets the ProtocolInfo.
    #[serde(rename = "ProtocolInfo", skip_serializing_if = "Option::is_none")]
    pub protocol_info: Option<String>,
    /// Gets or sets the TimelineOffsetSeconds.
    #[serde(
        rename = "TimelineOffsetSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeline_offset_seconds: Option<i32>,
    /// Gets or sets a value indicating whether RequiresPlainVideoItems.
    #[serde(
        rename = "RequiresPlainVideoItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_video_items: Option<bool>,
    /// Gets or sets a value indicating whether RequiresPlainFolders.
    #[serde(
        rename = "RequiresPlainFolders",
        skip_serializing_if = "Option::is_none"
    )]
    pub requires_plain_folders: Option<bool>,
    /// Gets or sets a value indicating whether EnableMSMediaReceiverRegistrar.
    #[serde(
        rename = "EnableMSMediaReceiverRegistrar",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_ms_media_receiver_registrar: Option<bool>,
    /// Gets or sets a value indicating whether IgnoreTranscodeByteRangeRequests.
    #[serde(
        rename = "IgnoreTranscodeByteRangeRequests",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_transcode_byte_range_requests: Option<bool>,
    /// Gets or sets the XmlRootAttributes.
    #[serde(rename = "XmlRootAttributes", skip_serializing_if = "Option::is_none")]
    pub xml_root_attributes: Option<Vec<crate::model::XmlAttribute>>,
    /// Gets or sets the direct play profiles.
    #[serde(rename = "DirectPlayProfiles", skip_serializing_if = "Option::is_none")]
    pub direct_play_profiles: Option<Vec<crate::model::DirectPlayProfile>>,
    /// Gets or sets the transcoding profiles.
    #[serde(
        rename = "TranscodingProfiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_profiles: Option<Vec<crate::model::TranscodingProfile>>,
    /// Gets or sets the ContainerProfiles.
    #[serde(rename = "ContainerProfiles", skip_serializing_if = "Option::is_none")]
    pub container_profiles: Option<Vec<crate::model::ContainerProfile>>,
    /// Gets or sets the CodecProfiles.
    #[serde(rename = "CodecProfiles", skip_serializing_if = "Option::is_none")]
    pub codec_profiles: Option<Vec<crate::model::CodecProfile>>,
    /// Gets or sets the ResponseProfiles.
    #[serde(rename = "ResponseProfiles", skip_serializing_if = "Option::is_none")]
    pub response_profiles: Option<Vec<crate::model::ResponseProfile>>,
    /// Gets or sets the SubtitleProfiles.
    #[serde(rename = "SubtitleProfiles", skip_serializing_if = "Option::is_none")]
    pub subtitle_profiles: Option<Vec<crate::model::SubtitleProfile>>,
}

impl DeviceProfile {
    /// Defines the MediaBrowser.Model.Dlna.DeviceProfile.
    pub fn new() -> DeviceProfile {
        DeviceProfile {
            name: None,
            id: None,
            identification: None,
            friendly_name: None,
            manufacturer: None,
            manufacturer_url: None,
            model_name: None,
            model_description: None,
            model_number: None,
            model_url: None,
            serial_number: None,
            enable_album_art_in_didl: None,
            enable_single_album_art_limit: None,
            enable_single_subtitle_limit: None,
            supported_media_types: None,
            user_id: None,
            album_art_pn: None,
            max_album_art_width: None,
            max_album_art_height: None,
            max_icon_width: None,
            max_icon_height: None,
            max_streaming_bitrate: None,
            max_static_bitrate: None,
            music_streaming_transcoding_bitrate: None,
            max_static_music_bitrate: None,
            sony_aggregation_flags: None,
            protocol_info: None,
            timeline_offset_seconds: None,
            requires_plain_video_items: None,
            requires_plain_folders: None,
            enable_ms_media_receiver_registrar: None,
            ignore_transcode_byte_range_requests: None,
            xml_root_attributes: None,
            direct_play_profiles: None,
            transcoding_profiles: None,
            container_profiles: None,
            codec_profiles: None,
            response_profiles: None,
            subtitle_profiles: None,
        }
    }
}
