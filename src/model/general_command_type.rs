use serde::{Deserialize, Serialize};

/// GeneralCommandType : This exists simply to identify a set of known commands.

/// This exists simply to identify a set of known commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GeneralCommandType {
    #[serde(rename = "MoveUp")]
    MoveUp,
    #[serde(rename = "MoveDown")]
    MoveDown,
    #[serde(rename = "MoveLeft")]
    MoveLeft,
    #[serde(rename = "MoveRight")]
    MoveRight,
    #[serde(rename = "PageUp")]
    PageUp,
    #[serde(rename = "PageDown")]
    PageDown,
    #[serde(rename = "PreviousLetter")]
    PreviousLetter,
    #[serde(rename = "NextLetter")]
    NextLetter,
    #[serde(rename = "ToggleOsd")]
    ToggleOsd,
    #[serde(rename = "ToggleContextMenu")]
    ToggleContextMenu,
    #[serde(rename = "Select")]
    Select,
    #[serde(rename = "Back")]
    Back,
    #[serde(rename = "TakeScreenshot")]
    TakeScreenshot,
    #[serde(rename = "SendKey")]
    SendKey,
    #[serde(rename = "SendString")]
    SendString,
    #[serde(rename = "GoHome")]
    GoHome,
    #[serde(rename = "GoToSettings")]
    GoToSettings,
    #[serde(rename = "VolumeUp")]
    VolumeUp,
    #[serde(rename = "VolumeDown")]
    VolumeDown,
    #[serde(rename = "Mute")]
    Mute,
    #[serde(rename = "Unmute")]
    Unmute,
    #[serde(rename = "ToggleMute")]
    ToggleMute,
    #[serde(rename = "SetVolume")]
    SetVolume,
    #[serde(rename = "SetAudioStreamIndex")]
    SetAudioStreamIndex,
    #[serde(rename = "SetSubtitleStreamIndex")]
    SetSubtitleStreamIndex,
    #[serde(rename = "ToggleFullscreen")]
    ToggleFullscreen,
    #[serde(rename = "DisplayContent")]
    DisplayContent,
    #[serde(rename = "GoToSearch")]
    GoToSearch,
    #[serde(rename = "DisplayMessage")]
    DisplayMessage,
    #[serde(rename = "SetRepeatMode")]
    SetRepeatMode,
    #[serde(rename = "ChannelUp")]
    ChannelUp,
    #[serde(rename = "ChannelDown")]
    ChannelDown,
    #[serde(rename = "Guide")]
    Guide,
    #[serde(rename = "ToggleStats")]
    ToggleStats,
    #[serde(rename = "PlayMediaSource")]
    PlayMediaSource,
    #[serde(rename = "PlayTrailers")]
    PlayTrailers,
    #[serde(rename = "SetShuffleQueue")]
    SetShuffleQueue,
    #[serde(rename = "PlayState")]
    PlayState,
    #[serde(rename = "PlayNext")]
    PlayNext,
    #[serde(rename = "ToggleOsdMenu")]
    ToggleOsdMenu,
    #[serde(rename = "Play")]
    Play,
}

impl ToString for GeneralCommandType {
    fn to_string(&self) -> String {
        match self {
            Self::MoveUp => String::from("MoveUp"),
            Self::MoveDown => String::from("MoveDown"),
            Self::MoveLeft => String::from("MoveLeft"),
            Self::MoveRight => String::from("MoveRight"),
            Self::PageUp => String::from("PageUp"),
            Self::PageDown => String::from("PageDown"),
            Self::PreviousLetter => String::from("PreviousLetter"),
            Self::NextLetter => String::from("NextLetter"),
            Self::ToggleOsd => String::from("ToggleOsd"),
            Self::ToggleContextMenu => String::from("ToggleContextMenu"),
            Self::Select => String::from("Select"),
            Self::Back => String::from("Back"),
            Self::TakeScreenshot => String::from("TakeScreenshot"),
            Self::SendKey => String::from("SendKey"),
            Self::SendString => String::from("SendString"),
            Self::GoHome => String::from("GoHome"),
            Self::GoToSettings => String::from("GoToSettings"),
            Self::VolumeUp => String::from("VolumeUp"),
            Self::VolumeDown => String::from("VolumeDown"),
            Self::Mute => String::from("Mute"),
            Self::Unmute => String::from("Unmute"),
            Self::ToggleMute => String::from("ToggleMute"),
            Self::SetVolume => String::from("SetVolume"),
            Self::SetAudioStreamIndex => String::from("SetAudioStreamIndex"),
            Self::SetSubtitleStreamIndex => String::from("SetSubtitleStreamIndex"),
            Self::ToggleFullscreen => String::from("ToggleFullscreen"),
            Self::DisplayContent => String::from("DisplayContent"),
            Self::GoToSearch => String::from("GoToSearch"),
            Self::DisplayMessage => String::from("DisplayMessage"),
            Self::SetRepeatMode => String::from("SetRepeatMode"),
            Self::ChannelUp => String::from("ChannelUp"),
            Self::ChannelDown => String::from("ChannelDown"),
            Self::Guide => String::from("Guide"),
            Self::ToggleStats => String::from("ToggleStats"),
            Self::PlayMediaSource => String::from("PlayMediaSource"),
            Self::PlayTrailers => String::from("PlayTrailers"),
            Self::SetShuffleQueue => String::from("SetShuffleQueue"),
            Self::PlayState => String::from("PlayState"),
            Self::PlayNext => String::from("PlayNext"),
            Self::ToggleOsdMenu => String::from("ToggleOsdMenu"),
            Self::Play => String::from("Play"),
        }
    }
}
