//! Branding operations
use http::request::Request;
use serde_json::from_str;

use crate::auth::AuthHeader;
use crate::error::SeascapeError::InvalidContent;
use crate::model::BrandingConfiguration;
use crate::{http::DataContentType, http::HttpClient};

use super::{ClientResult, Jellyfin};

impl Jellyfin {
    /// Gets branding configuration
    pub async fn branding_configuration(&self) -> ClientResult<BrandingConfiguration> {
        let url = format!("{}/{}", self.base_url, "branding/configuration");

        let request = Request::builder()
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
        }
    }
}
