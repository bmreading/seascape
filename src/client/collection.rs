//! Collection operations
use derive_builder::Builder;
use serde_json::from_str;

use crate::auth::AuthHeader;
use crate::error::SeascapeError::InvalidContent;
use crate::http::{DataContentType, HttpClient};
use crate::model::CollectionCreationResult;

use super::{ClientResult, Jellyfin};

impl Jellyfin {
    /// Create a new collection
    pub async fn create(
        &self,
        params: &CollectionParams,
    ) -> ClientResult<CollectionCreationResult> {
        let url = format!("{}/{}", self.base_url, "collections");

        let ids = params.ids.as_ref().map(|x| x.join(","));
        let is_locked = params.is_locked.map(|x| x.to_string());

        let params = crate::build_map!(
            optional "name": params.name.as_deref(),
            optional "ids": ids.as_deref(),
            optional "parent_id": params.parent_id.as_deref(),
            optional "is_locked": is_locked.as_deref(),
        );

        let request = http::request::Request::builder()
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

    /// Add items to a collection
    pub async fn add_items(&self, collection_id: &str, ids_to_add: &Vec<&str>) -> ClientResult<()> {
        let url = format!(
            "{}/{}/{}/{}",
            self.base_url, "collections", collection_id, "items"
        );

        let joined_ids_to_add = ids_to_add.join(",");

        let params = crate::build_map!(
            "ids": joined_ids_to_add.as_ref(),
        );

        let request = http::request::Request::builder()
            .uri(url)
            .method("POST")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .header("content-length", 0)
            .body(None)?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::NoContent => Ok(()),
            DataContentType::TextContent(_) => Err(InvalidContent),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
        }
    }
}

#[derive(Builder, Clone, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct CollectionParams {
    name: Option<String>,
    ids: Option<Vec<String>>,
    parent_id: Option<String>,
    is_locked: Option<bool>,
}
