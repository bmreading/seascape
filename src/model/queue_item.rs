use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueItem {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
}

impl QueueItem {
    pub fn new() -> QueueItem {
        QueueItem {
            id: None,
            playlist_item_id: None,
        }
    }
}
