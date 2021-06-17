use serde::{Deserialize, Serialize};

/// LocationType : Enum LocationType.

/// Enum LocationType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "FileSystem")]
    FileSystem,
    #[serde(rename = "Remote")]
    Remote,
    #[serde(rename = "Virtual")]
    _Virtual,
    #[serde(rename = "Offline")]
    Offline,
}

impl ToString for LocationType {
    fn to_string(&self) -> String {
        match self {
            Self::FileSystem => String::from("FileSystem"),
            Self::Remote => String::from("Remote"),
            Self::_Virtual => String::from("Virtual"),
            Self::Offline => String::from("Offline"),
        }
    }
}
