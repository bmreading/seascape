use serde::{Deserialize, Serialize};

/// Data regarding a user item.
#[derive(Clone, Debug, Default,PartialEq, Serialize, Deserialize)]
pub struct UserItemDataDto {
    /// The rating.
    #[serde(rename = "Rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    /// The played percentage.
    #[serde(rename = "PlayedPercentage", skip_serializing_if = "Option::is_none")]
    pub played_percentage: Option<f64>,
    /// The unplayed item count.
    #[serde(rename = "UnplayedItemCount", skip_serializing_if = "Option::is_none")]
    pub unplayed_item_count: Option<i32>,
    /// The playback position ticks.
    #[serde(
        rename = "PlaybackPositionTicks",
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_position_ticks: Option<i64>,
    /// The play count.
    #[serde(rename = "PlayCount", skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i32>,
    /// A value indicating whether this instance is favorite.
    #[serde(rename = "IsFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    /// A value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is likes.
    #[serde(rename = "Likes", skip_serializing_if = "Option::is_none")]
    pub likes: Option<bool>,
    /// The last played date.
    #[serde(rename = "LastPlayedDate", skip_serializing_if = "Option::is_none")]
    pub last_played_date: Option<String>,
    /// A value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is played.
    #[serde(rename = "Played", skip_serializing_if = "Option::is_none")]
    pub played: Option<bool>,
    /// The key.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The item identifier.
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}