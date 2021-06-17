use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubtitleDeliveryMethod {
    #[serde(rename = "Encode")]
    Encode,
    #[serde(rename = "Embed")]
    Embed,
    #[serde(rename = "External")]
    External,
    #[serde(rename = "Hls")]
    Hls,
}

impl ToString for SubtitleDeliveryMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Encode => String::from("Encode"),
            Self::Embed => String::from("Embed"),
            Self::External => String::from("External"),
            Self::Hls => String::from("Hls"),
        }
    }
}
