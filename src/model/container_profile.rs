use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProfile {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Box<crate::model::DlnaProfileType>>,
    #[serde(rename = "Conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::model::ProfileCondition>>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
}

impl ContainerProfile {
    pub fn new() -> ContainerProfile {
        ContainerProfile {
            _type: None,
            conditions: None,
            container: None,
        }
    }
}
