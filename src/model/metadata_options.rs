//! Metadata Options
use serde::{Deserialize, Serialize};

/// Represents options for metadata
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataOptions {
    #[serde(rename = "ItemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(rename = "DisabledMetadataSavers", skip_serializing_if = "Option::is_none")]
    pub disabled_metadata_savers: Option<Vec<String>>,
    #[serde(rename = "LocalMetadataReaderOrder", skip_serializing_if = "Option::is_none")]
    pub local_metadata_reader_order: Option<Vec<String>>,
    #[serde(rename = "DisabledMetadataFetchers", skip_serializing_if = "Option::is_none")]
    pub disabled_metadata_fetchers: Option<Vec<String>>,
    #[serde(rename = "MetadataFetcherOrder", skip_serializing_if = "Option::is_none")]
    pub metadata_fetcher_order: Option<Vec<String>>,
    #[serde(rename = "DisabledImageFetchers", skip_serializing_if = "Option::is_none")]
    pub disabled_image_fetchers: Option<Vec<String>>,
    #[serde(rename = "ImageFetcherOrder", skip_serializing_if = "Option::is_none")]
    pub image_fetcher_order: Option<Vec<String>>,
}

impl MetadataOptions {
    /// Class MetadataOptions.
    pub fn new() -> MetadataOptions {
        MetadataOptions {
            item_type: None,
            disabled_metadata_savers: None,
            local_metadata_reader_order: None,
            disabled_metadata_fetchers: None,
            metadata_fetcher_order: None,
            disabled_image_fetchers: None,
            image_fetcher_order: None,
        }
    }
}


