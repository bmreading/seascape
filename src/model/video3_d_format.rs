use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Video3DFormat {
    #[serde(rename = "HalfSideBySide")]
    HalfSideBySide,
    #[serde(rename = "FullSideBySide")]
    FullSideBySide,
    #[serde(rename = "FullTopAndBottom")]
    FullTopAndBottom,
    #[serde(rename = "HalfTopAndBottom")]
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    Mvc,
}

impl ToString for Video3DFormat {
    fn to_string(&self) -> String {
        match self {
            Self::HalfSideBySide => String::from("HalfSideBySide"),
            Self::FullSideBySide => String::from("FullSideBySide"),
            Self::FullTopAndBottom => String::from("FullTopAndBottom"),
            Self::HalfTopAndBottom => String::from("HalfTopAndBottom"),
            Self::Mvc => String::from("MVC"),
        }
    }
}
