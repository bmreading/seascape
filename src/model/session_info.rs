use serde::{Deserialize, Serialize};

/// Information about a session
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "PlayState", skip_serializing_if = "Option::is_none")]
    pub play_state: Option<Box<crate::model::PlayerStateInfo>>,
    #[serde(rename = "AdditionalUsers", skip_serializing_if = "Option::is_none")]
    pub additional_users: Option<Vec<crate::model::SessionUserInfo>>,
    #[serde(rename = "Capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<crate::model::ClientCapabilities>>,
    /// The remote end point.
    #[serde(rename = "RemoteEndPoint", skip_serializing_if = "Option::is_none")]
    pub remote_end_point: Option<String>,
    /// The playable media types.
    #[serde(rename = "PlayableMediaTypes", skip_serializing_if = "Option::is_none")]
    pub playable_media_types: Option<Vec<String>>,
    /// The id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The user id.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The username.
    #[serde(rename = "UserName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// The type of the client.
    #[serde(rename = "Client", skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    /// The last activity date.
    #[serde(rename = "LastActivityDate", skip_serializing_if = "Option::is_none")]
    pub last_activity_date: Option<String>,
    /// The last playback check in.
    #[serde(
        rename = "LastPlaybackCheckIn",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_playback_check_in: Option<String>,
    /// The name of the device.
    #[serde(rename = "DeviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// The type of the device.
    #[serde(rename = "DeviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// The now playing item.
    #[serde(rename = "NowPlayingItem", skip_serializing_if = "Option::is_none")]
    pub now_playing_item: Option<Box<crate::model::BaseItemDto>>,
    /// BaseItem struct
    #[serde(rename = "FullNowPlayingItem", skip_serializing_if = "Option::is_none")]
    pub full_now_playing_item: Option<Box<crate::model::BaseItem>>,
    /// This holds information about a BaseItem in a format that is convenient for the server API.
    #[serde(rename = "NowViewingItem", skip_serializing_if = "Option::is_none")]
    pub now_viewing_item: Option<Box<crate::model::BaseItemDto>>,
    /// The device id.
    #[serde(rename = "DeviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// The application version.
    #[serde(rename = "ApplicationVersion", skip_serializing_if = "Option::is_none")]
    pub application_version: Option<String>,
    #[serde(rename = "TranscodingInfo", skip_serializing_if = "Option::is_none")]
    pub transcoding_info: Option<Box<crate::model::TranscodingInfo>>,
    /// Gets a value indicating whether this instance is active.
    #[serde(rename = "IsActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(
        rename = "SupportsMediaControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_media_control: Option<bool>,
    #[serde(
        rename = "SupportsRemoteControl",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_remote_control: Option<bool>,
    #[serde(rename = "NowPlayingQueue", skip_serializing_if = "Option::is_none")]
    pub now_playing_queue: Option<Vec<crate::model::QueueItem>>,
    #[serde(
        rename = "HasCustomDeviceName",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_custom_device_name: Option<bool>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(
        rename = "UserPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_primary_image_tag: Option<String>,
    /// The supported commands.
    #[serde(rename = "SupportedCommands", skip_serializing_if = "Option::is_none")]
    pub supported_commands: Option<Vec<crate::model::GeneralCommandType>>,
}

impl SessionInfo {
    /// Class SessionInfo.
    pub fn new() -> SessionInfo {
        SessionInfo {
            play_state: None,
            additional_users: None,
            capabilities: None,
            remote_end_point: None,
            playable_media_types: None,
            id: None,
            user_id: None,
            user_name: None,
            client: None,
            last_activity_date: None,
            last_playback_check_in: None,
            device_name: None,
            device_type: None,
            now_playing_item: None,
            full_now_playing_item: None,
            now_viewing_item: None,
            device_id: None,
            application_version: None,
            transcoding_info: None,
            is_active: None,
            supports_media_control: None,
            supports_remote_control: None,
            now_playing_queue: None,
            has_custom_device_name: None,
            playlist_item_id: None,
            server_id: None,
            user_primary_image_tag: None,
            supported_commands: None,
        }
    }
}
