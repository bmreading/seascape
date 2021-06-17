use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayAccess {
    #[serde(rename = "Full")]
    Full,
    #[serde(rename = "None")]
    None,
}

impl ToString for PlayAccess {
    fn to_string(&self) -> String {
        match self {
            Self::Full => String::from("Full"),
            Self::None => String::from("None"),
        }
    }
}
