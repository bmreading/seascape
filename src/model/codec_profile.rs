use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CodecProfile {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::CodecType>>,
    #[serde(rename = "Conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::model::ProfileCondition>>,
    #[serde(rename = "ApplyConditions", skip_serializing_if = "Option::is_none")]
    pub apply_conditions: Option<Vec<crate::model::ProfileCondition>>,
    #[serde(rename = "Codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
}
