use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProfileConditionType {
    #[serde(rename = "Equals")]
    Equals,
    #[serde(rename = "NotEquals")]
    NotEquals,
    #[serde(rename = "LessThanEqual")]
    LessThanEqual,
    #[serde(rename = "GreaterThanEqual")]
    GreaterThanEqual,
    #[serde(rename = "EqualsAny")]
    EqualsAny,
}

impl ToString for ProfileConditionType {
    fn to_string(&self) -> String {
        match self {
            Self::Equals => String::from("Equals"),
            Self::NotEquals => String::from("NotEquals"),
            Self::LessThanEqual => String::from("LessThanEqual"),
            Self::GreaterThanEqual => String::from("GreaterThanEqual"),
            Self::EqualsAny => String::from("EqualsAny"),
        }
    }
}
