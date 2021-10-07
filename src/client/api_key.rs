//! API key operations
use serde_json::from_str;

use crate::auth::AuthHeader;
use crate::error::SeascapeError::InvalidContent;
use crate::http::{DataContentType, HttpClient, QueryParamMap, RequestBuilder};
use crate::model::BaseItemDto;

use super::{ClientResult, ItemResponse, Jellyfin};

impl Jellyfin {
    /// Retrieves API keys
    pub async fn keys(&self) -> ClientResult<ItemResponse<BaseItemDto>> {
        let url = format!("{}/{}", self.base_url, "auth/keys");

        let request = RequestBuilder::new()
            .uri(url)
            .method("GET")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .body(None)?;

        let response = self.http_client_type.send(&request, None).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
            DataContentType::NoContent => Err(InvalidContent),
        }
    }

    /// Adds an API key
    pub async fn add_key(&self, key: &str) -> ClientResult<ItemResponse<BaseItemDto>> {
        let url = format!("{}/{}", self.base_url, "auth/keys");

        let mut params = QueryParamMap::new();
        params.insert("key", key);

        let request = RequestBuilder::new()
            .uri(url)
            .method("POST")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .body(None)?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
            DataContentType::NoContent => Err(InvalidContent),
        }
    }

    /// Removes an API key
    pub async fn remove_key(&self, app: &str) -> ClientResult<ItemResponse<BaseItemDto>> {
        let url = format!("{}/{}", self.base_url, "auth/keys");

        let mut params = QueryParamMap::new();
        params.insert("app", app);

        let request = RequestBuilder::new()
            .uri(url)
            .method("DELETE")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .body(None)?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
            DataContentType::NoContent => Err(InvalidContent),
        }
    }
}
