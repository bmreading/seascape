use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaProtocol {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Http")]
    Http,
    #[serde(rename = "Rtmp")]
    Rtmp,
    #[serde(rename = "Rtsp")]
    Rtsp,
    #[serde(rename = "Udp")]
    Udp,
    #[serde(rename = "Rtp")]
    Rtp,
    #[serde(rename = "Ftp")]
    Ftp,
}

impl ToString for MediaProtocol {
    fn to_string(&self) -> String {
        match self {
            Self::File => String::from("File"),
            Self::Http => String::from("Http"),
            Self::Rtmp => String::from("Rtmp"),
            Self::Rtsp => String::from("Rtsp"),
            Self::Udp => String::from("Udp"),
            Self::Rtp => String::from("Rtp"),
            Self::Ftp => String::from("Ftp"),
        }
    }
}
