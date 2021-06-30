use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProfileCondition {
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<crate::model::ProfileConditionType>>,
    #[serde(rename = "Property", skip_serializing_if = "Option::is_none")]
    pub property: Option<Box<crate::model::ProfileConditionValue>>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "IsRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
}
