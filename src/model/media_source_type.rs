use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaSourceType {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Grouping")]
    Grouping,
    #[serde(rename = "Placeholder")]
    Placeholder,
}

impl ToString for MediaSourceType {
    fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("Default"),
            Self::Grouping => String::from("Grouping"),
            Self::Placeholder => String::from("Placeholder"),
        }
    }
}
