use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EncodingContext {
    #[serde(rename = "Streaming")]
    Streaming,
    #[serde(rename = "Static")]
    _Static,
}

impl ToString for EncodingContext {
    fn to_string(&self) -> String {
        match self {
            Self::Streaming => String::from("Streaming"),
            Self::_Static => String::from("Static"),
        }
    }
}
