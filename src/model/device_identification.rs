use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceIdentification {
    /// Gets or sets the name of the friendly.
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Gets or sets the model number.
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
    /// Gets or sets the serial number.
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Gets or sets the name of the model.
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// Gets or sets the model description.
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    /// Gets or sets the model URL.
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option::is_none")]
    pub model_url: Option<String>,
    /// Gets or sets the manufacturer.
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Gets or sets the manufacturer URL.
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option::is_none")]
    pub manufacturer_url: Option<String>,
    /// Gets or sets the headers.
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<crate::model::HttpHeaderInfo>>,
}

impl DeviceIdentification {
    pub fn new() -> DeviceIdentification {
        DeviceIdentification {
            friendly_name: None,
            model_number: None,
            serial_number: None,
            model_name: None,
            model_description: None,
            model_url: None,
            manufacturer: None,
            manufacturer_url: None,
            headers: None,
        }
    }
}
