use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayMethod {
    #[serde(rename = "Transcode")]
    Transcode,
    #[serde(rename = "DirectStream")]
    DirectStream,
    #[serde(rename = "DirectPlay")]
    DirectPlay,
}

impl ToString for PlayMethod {
    fn to_string(&self) -> String {
        match self {
            Self::Transcode => String::from("Transcode"),
            Self::DirectStream => String::from("DirectStream"),
            Self::DirectPlay => String::from("DirectPlay"),
        }
    }
}
