use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransportStreamTimestamp {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Zero")]
    Zero,
    #[serde(rename = "Valid")]
    Valid,
}

impl ToString for TransportStreamTimestamp {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("None"),
            Self::Zero => String::from("Zero"),
            Self::Valid => String::from("Valid"),
        }
    }
}
