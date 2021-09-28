//! Channel operations
use derive_builder::Builder;
use http::request::Request;
use serde_json::from_str;

use crate::auth::AuthHeader;
use crate::http::{DataContentType, HttpClient};
use crate::error::SeascapeError::InvalidContent;
use crate::model::{BaseItemDto, ChannelFeatures};

use super::{Jellyfin, ClientResult};

impl Jellyfin {
    /// Get available channels
    pub async fn channels(&self, channel_query: &ChannelQuery) -> ClientResult<BaseItemDto> {
        let url = format!("{}/{}", self.base_url, "channels");

        let start_index = channel_query.start_index.map(|x| x.to_string());
        let limit = channel_query.limit.map(|x| x.to_string());
        let supports_latest_items = channel_query.supports_latest_items.map(|x| x.to_string());
        let supports_media_deletion = channel_query.supports_media_deletion.map(|x| x.to_string());
        let is_favorite = channel_query.is_favorite.map(|x| x.to_string());

        let params = build_map!(
            optional "userId": channel_query.user_id.as_deref(),
            optional "startIndex": start_index.as_deref(),
            optional "limit": limit.as_deref(),
            optional "supportsLatestItems": supports_latest_items.as_deref(),
            optional "supportsMediaDeletion": supports_media_deletion.as_deref(),
            optional "isFavorite": is_favorite.as_deref(),
        );

        let request = Request::builder()
            .uri(url)
            .method("GET")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value()
            )
            .body(None)?;
        
        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
        }
    }

    pub async fn channel_features(&self, channel_id: &str) -> ClientResult<ChannelFeatures> {
        let url = format!("{}/{}/{}/{}", self.base_url, "channels", channel_id, "features");
        
        let request = Request::builder()
            .uri(url)
            .method("GET")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value()
            )
            .body(None)?;
        
        let response = self.http_client_type.send(&request, None).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent)
        }
    }
}

#[derive(Builder, Clone, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ChannelQuery {
    user_id: Option<String>,
    start_index: Option<i32>,
    limit: Option<i32>,
    supports_latest_items: Option<bool>,
    supports_media_deletion: Option<bool>,
    is_favorite: Option<bool>,
}