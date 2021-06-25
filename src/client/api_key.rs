//! API key operations
use crate::auth::AuthHeader;
use crate::http::{HttpClient, QueryParamMap, RequestBuilder};
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
            .body("".to_string())?;

        let response = self.http_client_type.send(&request, None).await?;
        let response_body = response.body();

        Ok(serde_json::from_str(response_body.as_str())?)
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
            .body("".to_string())?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;
        let response_body = response.body();

        Ok(serde_json::from_str(response_body.as_str())?)
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
            .body("".to_string())?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;
        let response_body = response.body();

        Ok(serde_json::from_str(response_body.as_str())?)
    }
}
