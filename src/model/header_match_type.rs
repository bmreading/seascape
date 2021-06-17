use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HeaderMatchType {
    #[serde(rename = "Equals")]
    Equals,
    #[serde(rename = "Regex")]
    Regex,
    #[serde(rename = "Substring")]
    Substring,
}

impl ToString for HeaderMatchType {
    fn to_string(&self) -> String {
        match self {
            Self::Equals => String::from("Equals"),
            Self::Regex => String::from("Regex"),
            Self::Substring => String::from("Substring"),
        }
    }
}
