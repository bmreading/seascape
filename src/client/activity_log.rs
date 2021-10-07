//! Activity log operations
use http::header;
use serde_json::from_str;

use crate::auth::AuthHeader;
use crate::error::SeascapeError::InvalidContent;
use crate::http::{DataContentType, HttpClient};
use crate::model::ActivityLogEntry;

use super::{ClientResult, ItemResponse, Jellyfin};

impl Jellyfin {
    // Retrieves activity log entries
    pub async fn activity_log_entries(
        &self,
        start_index: Option<i32>,
        limit: Option<i32>,
        min_date: Option<&str>,
        has_user_id: Option<bool>,
    ) -> ClientResult<ItemResponse<ActivityLogEntry>> {
        let url = format!("{}/{}", self.base_url, "system/activitylog/entries");

        let request = http::request::Request::builder()
            .uri(url)
            .method("GET")
            .header(header::CONTENT_TYPE, "application/json")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .body(None)?;

        let start_index = start_index.map(|i| i.to_string());
        let limit = limit.map(|i| i.to_string());
        let has_user_id = has_user_id.map(|b| b.to_string());

        let params = build_map!(
            optional "startIndex": start_index.as_deref(),
            optional "limit": limit.as_deref(),
            optional "minDate": min_date.as_deref(),
            optional "hasUserId": has_user_id.as_deref(),
        );

        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::TextContent(c) => Ok(from_str(c)?),
            DataContentType::BinaryContent(_) => Err(InvalidContent),
            DataContentType::NoContent => Err(InvalidContent),
        }
    }
}
