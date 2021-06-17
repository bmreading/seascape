use serde::{Deserialize, Serialize};

/// IsoType : Enum IsoType.

/// Enum IsoType.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsoType {
    #[serde(rename = "Dvd")]
    Dvd,
    #[serde(rename = "BluRay")]
    BluRay,
}

impl ToString for IsoType {
    fn to_string(&self) -> String {
        match self {
            Self::Dvd => String::from("Dvd"),
            Self::BluRay => String::from("BluRay"),
        }
    }
}
