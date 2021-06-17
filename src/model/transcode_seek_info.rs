use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscodeSeekInfo {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "Bytes")]
    Bytes,
}

impl ToString for TranscodeSeekInfo {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => String::from("Auto"),
            Self::Bytes => String::from("Bytes"),
        }
    }
}
