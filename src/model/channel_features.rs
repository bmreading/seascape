//! Items specific to a channel's features
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// A channel's features
pub struct ChannelFeatures {
    /// The name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The identifier.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A value indicating whether this instance can search.
    #[serde(rename = "CanSearch", skip_serializing_if = "Option::is_none")]
    pub can_search: Option<bool>,
    /// The media types.
    #[serde(rename = "MediaTypes", skip_serializing_if = "Option::is_none")]
    pub media_types: Option<Vec<ChannelMediaType>>,
    /// The content types.
    #[serde(rename = "ContentTypes", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<ChannelMediaContentType>>,
    /// The maximum number of records the channel allows retrieving at a time.
    #[serde(rename = "MaxPageSize", skip_serializing_if = "Option::is_none")]
    pub max_page_size: Option<i32>,
    /// The automatic refresh levels.
    #[serde(rename = "AutoRefreshLevels", skip_serializing_if = "Option::is_none")]
    pub auto_refresh_levels: Option<i32>,
    /// The default sort orders.
    #[serde(rename = "DefaultSortFields", skip_serializing_if = "Option::is_none")]
    pub default_sort_fields: Option<Vec<ChannelItemSortField>>,
    /// Indicates if a sort ascending/descending toggle is supported or not.
    #[serde(
        rename = "SupportsSortOrderToggle",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_sort_order_toggle: Option<bool>,
    /// A value indicating whether latest media is supported or not.
    #[serde(
        rename = "SupportsLatestMedia",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_latest_media: Option<bool>,
    /// A value indicating whether this instance can filter.
    #[serde(rename = "CanFilter", skip_serializing_if = "Option::is_none")]
    pub can_filter: Option<bool>,
    /// A value indicating whether content downloading is supported or not.
    #[serde(
        rename = "SupportsContentDownloading",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_content_downloading: Option<bool>,
}

/// The type of media a channel can have
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChannelMediaType {
    Audio,
    Video,
    Photo,
}

/// The type of media content a channel can have
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChannelMediaContentType {
    Clip,
    Podcast,
    Trailer,
    Movie,
    Episode,
    Song,
    MovieExtra,
    TvExtra,
}

/// A specified method to sort an channel's items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChannelItemSortField {
    Name,
    CommunityRating,
    PremiereDate,
    DateCreated,
    Runtime,
    PlayCount,
    CommunityPlayCount,
}
