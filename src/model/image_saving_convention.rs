use serde::{Deserialize, Serialize};

/// A convention to save
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageSavingConvention {
    #[serde(rename = "Legacy")]
    Legacy,
    #[serde(rename = "Compatible")]
    Compatible,
}

impl ToString for ImageSavingConvention {
    fn to_string(&self) -> String {
        match self {
            Self::Legacy => String::from("Legacy"),
            Self::Compatible => String::from("Compatible"),
        }
    }
}
