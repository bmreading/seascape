use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageOrientation {
    #[serde(rename = "TopLeft")]
    TopLeft,
    #[serde(rename = "TopRight")]
    TopRight,
    #[serde(rename = "BottomRight")]
    BottomRight,
    #[serde(rename = "BottomLeft")]
    BottomLeft,
    #[serde(rename = "LeftTop")]
    LeftTop,
    #[serde(rename = "RightTop")]
    RightTop,
    #[serde(rename = "RightBottom")]
    RightBottom,
    #[serde(rename = "LeftBottom")]
    LeftBottom,
}

impl ToString for ImageOrientation {
    fn to_string(&self) -> String {
        match self {
            Self::TopLeft => String::from("TopLeft"),
            Self::TopRight => String::from("TopRight"),
            Self::BottomRight => String::from("BottomRight"),
            Self::BottomLeft => String::from("BottomLeft"),
            Self::LeftTop => String::from("LeftTop"),
            Self::RightTop => String::from("RightTop"),
            Self::RightBottom => String::from("RightBottom"),
            Self::LeftBottom => String::from("LeftBottom"),
        }
    }
}
