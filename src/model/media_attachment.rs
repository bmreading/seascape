use serde::{Deserialize, Serialize};
/// MediaAttachment : Class MediaAttachment.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAttachment {
    /// Gets or sets the codec.
    #[serde(rename = "Codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// Gets or sets the codec tag.
    #[serde(rename = "CodecTag", skip_serializing_if = "Option::is_none")]
    pub codec_tag: Option<String>,
    /// Gets or sets the comment.
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Gets or sets the index.
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Gets or sets the filename.
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Gets or sets the MIME type.
    #[serde(rename = "MimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Gets or sets the delivery URL.
    #[serde(rename = "DeliveryUrl", skip_serializing_if = "Option::is_none")]
    pub delivery_url: Option<String>,
}

impl MediaAttachment {
    /// Class MediaAttachment.
    pub fn new() -> MediaAttachment {
        MediaAttachment {
            codec: None,
            codec_tag: None,
            comment: None,
            index: None,
            file_name: None,
            mime_type: None,
            delivery_url: None,
        }
    }
}
