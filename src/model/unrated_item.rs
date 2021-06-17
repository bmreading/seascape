use serde::{Deserialize, Serialize};

/// UnratedItem : An enum representing an unrated item.

/// An enum representing an unrated item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnratedItem {
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Music")]
    Music,
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "LiveTvChannel")]
    LiveTvChannel,
    #[serde(rename = "LiveTvProgram")]
    LiveTvProgram,
    #[serde(rename = "ChannelContent")]
    ChannelContent,
    #[serde(rename = "Other")]
    Other,
}

impl ToString for UnratedItem {
    fn to_string(&self) -> String {
        match self {
            Self::Movie => String::from("Movie"),
            Self::Trailer => String::from("Trailer"),
            Self::Series => String::from("Series"),
            Self::Music => String::from("Music"),
            Self::Book => String::from("Book"),
            Self::LiveTvChannel => String::from("LiveTvChannel"),
            Self::LiveTvProgram => String::from("LiveTvProgram"),
            Self::ChannelContent => String::from("ChannelContent"),
            Self::Other => String::from("Other"),
        }
    }
}
