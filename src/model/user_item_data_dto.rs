use serde::{Deserialize, Serialize};

/// UserItemDataDto : Class UserItemDataDto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserItemDataDto {
    /// Gets or sets the rating.
    #[serde(rename = "Rating", skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    /// Gets or sets the played percentage.
    #[serde(rename = "PlayedPercentage", skip_serializing_if = "Option::is_none")]
    pub played_percentage: Option<f64>,
    /// Gets or sets the unplayed item count.
    #[serde(rename = "UnplayedItemCount", skip_serializing_if = "Option::is_none")]
    pub unplayed_item_count: Option<i32>,
    /// Gets or sets the playback position ticks.
    #[serde(
        rename = "PlaybackPositionTicks",
        skip_serializing_if = "Option::is_none"
    )]
    pub playback_position_ticks: Option<i64>,
    /// Gets or sets the play count.
    #[serde(rename = "PlayCount", skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i32>,
    /// Gets or sets a value indicating whether this instance is favorite.
    #[serde(rename = "IsFavorite", skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<bool>,
    /// Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is likes.
    #[serde(rename = "Likes", skip_serializing_if = "Option::is_none")]
    pub likes: Option<bool>,
    /// Gets or sets the last played date.
    #[serde(rename = "LastPlayedDate", skip_serializing_if = "Option::is_none")]
    pub last_played_date: Option<String>,
    /// Gets or sets a value indicating whether this MediaBrowser.Model.Dto.UserItemDataDto is played.
    #[serde(rename = "Played", skip_serializing_if = "Option::is_none")]
    pub played: Option<bool>,
    /// Gets or sets the key.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Gets or sets the item identifier.
    #[serde(rename = "ItemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

impl UserItemDataDto {
    /// Class UserItemDataDto.
    pub fn new() -> UserItemDataDto {
        UserItemDataDto {
            rating: None,
            played_percentage: None,
            unplayed_item_count: None,
            playback_position_ticks: None,
            play_count: None,
            is_favorite: None,
            likes: None,
            last_played_date: None,
            played: None,
            key: None,
            item_id: None,
        }
    }
}
