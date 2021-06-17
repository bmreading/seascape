use serde::{Deserialize, Serialize};

/// ChannelType : Enum ChannelType.

/// Enum ChannelType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    #[serde(rename = "TV")]
    TV,
    #[serde(rename = "Radio")]
    Radio,
}

impl ToString for ChannelType {
    fn to_string(&self) -> String {
        match self {
            Self::TV => String::from("TV"),
            Self::Radio => String::from("Radio"),
        }
    }
}
