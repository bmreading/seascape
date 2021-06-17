use serde::{Deserialize, Serialize};

/// MetadataField : Enum MetadataFields.

/// Enum MetadataFields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetadataField {
    #[serde(rename = "Cast")]
    Cast,
    #[serde(rename = "Genres")]
    Genres,
    #[serde(rename = "ProductionLocations")]
    ProductionLocations,
    #[serde(rename = "Studios")]
    Studios,
    #[serde(rename = "Tags")]
    Tags,
    #[serde(rename = "Name")]
    Name,
    #[serde(rename = "Overview")]
    Overview,
    #[serde(rename = "Runtime")]
    Runtime,
    #[serde(rename = "OfficialRating")]
    OfficialRating,
}

impl ToString for MetadataField {
    fn to_string(&self) -> String {
        match self {
            Self::Cast => String::from("Cast"),
            Self::Genres => String::from("Genres"),
            Self::ProductionLocations => String::from("ProductionLocations"),
            Self::Studios => String::from("Studios"),
            Self::Tags => String::from("Tags"),
            Self::Name => String::from("Name"),
            Self::Overview => String::from("Overview"),
            Self::Runtime => String::from("Runtime"),
            Self::OfficialRating => String::from("OfficialRating"),
        }
    }
}
