use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DeviceIdentification {
    /// The friendly name.
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The model number.
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option::is_none")]
    pub model_number: Option<String>,
    /// The serial number.
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The name of the model.
    #[serde(rename = "ModelName", skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    /// The model description.
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option::is_none")]
    pub model_description: Option<String>,
    /// The model URL.
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option::is_none")]
    pub model_url: Option<String>,
    /// The manufacturer.
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The manufacturer URL.
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option::is_none")]
    pub manufacturer_url: Option<String>,
    /// The headers.
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<crate::model::HttpHeaderInfo>>,
}
