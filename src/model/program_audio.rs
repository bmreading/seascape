use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProgramAudio {
    #[serde(rename = "Mono")]
    Mono,
    #[serde(rename = "Stereo")]
    Stereo,
    #[serde(rename = "Dolby")]
    Dolby,
    #[serde(rename = "DolbyDigital")]
    DolbyDigital,
    #[serde(rename = "Thx")]
    Thx,
    #[serde(rename = "Atmos")]
    Atmos,
}

impl ToString for ProgramAudio {
    fn to_string(&self) -> String {
        match self {
            Self::Mono => String::from("Mono"),
            Self::Stereo => String::from("Stereo"),
            Self::Dolby => String::from("Dolby"),
            Self::DolbyDigital => String::from("DolbyDigital"),
            Self::Thx => String::from("Thx"),
            Self::Atmos => String::from("Atmos"),
        }
    }
}
