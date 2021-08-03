//! Items related to branding configuration

use serde::{Deserialize, Serialize};

/// Branding configuration
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BrandingConfiguration {
    /// The login disclaimer
    #[serde(rename = "LoginDisclaimer")]
    login_disclaimer: String,
    /// Custom CSS
    #[serde(rename = "CustomCss")]
    custom_css: String,
}
