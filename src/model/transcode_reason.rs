use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscodeReason {
    #[serde(rename = "ContainerNotSupported")]
    ContainerNotSupported,
    #[serde(rename = "VideoCodecNotSupported")]
    VideoCodecNotSupported,
    #[serde(rename = "AudioCodecNotSupported")]
    AudioCodecNotSupported,
    #[serde(rename = "ContainerBitrateExceedsLimit")]
    ContainerBitrateExceedsLimit,
    #[serde(rename = "AudioBitrateNotSupported")]
    AudioBitrateNotSupported,
    #[serde(rename = "AudioChannelsNotSupported")]
    AudioChannelsNotSupported,
    #[serde(rename = "VideoResolutionNotSupported")]
    VideoResolutionNotSupported,
    #[serde(rename = "UnknownVideoStreamInfo")]
    UnknownVideoStreamInfo,
    #[serde(rename = "UnknownAudioStreamInfo")]
    UnknownAudioStreamInfo,
    #[serde(rename = "AudioProfileNotSupported")]
    AudioProfileNotSupported,
    #[serde(rename = "AudioSampleRateNotSupported")]
    AudioSampleRateNotSupported,
    #[serde(rename = "AnamorphicVideoNotSupported")]
    AnamorphicVideoNotSupported,
    #[serde(rename = "InterlacedVideoNotSupported")]
    InterlacedVideoNotSupported,
    #[serde(rename = "SecondaryAudioNotSupported")]
    SecondaryAudioNotSupported,
    #[serde(rename = "RefFramesNotSupported")]
    RefFramesNotSupported,
    #[serde(rename = "VideoBitDepthNotSupported")]
    VideoBitDepthNotSupported,
    #[serde(rename = "VideoBitrateNotSupported")]
    VideoBitrateNotSupported,
    #[serde(rename = "VideoFramerateNotSupported")]
    VideoFramerateNotSupported,
    #[serde(rename = "VideoLevelNotSupported")]
    VideoLevelNotSupported,
    #[serde(rename = "VideoProfileNotSupported")]
    VideoProfileNotSupported,
    #[serde(rename = "AudioBitDepthNotSupported")]
    AudioBitDepthNotSupported,
    #[serde(rename = "SubtitleCodecNotSupported")]
    SubtitleCodecNotSupported,
    #[serde(rename = "DirectPlayError")]
    DirectPlayError,
}

impl ToString for TranscodeReason {
    fn to_string(&self) -> String {
        match self {
            Self::ContainerNotSupported => String::from("ContainerNotSupported"),
            Self::VideoCodecNotSupported => String::from("VideoCodecNotSupported"),
            Self::AudioCodecNotSupported => String::from("AudioCodecNotSupported"),
            Self::ContainerBitrateExceedsLimit => String::from("ContainerBitrateExceedsLimit"),
            Self::AudioBitrateNotSupported => String::from("AudioBitrateNotSupported"),
            Self::AudioChannelsNotSupported => String::from("AudioChannelsNotSupported"),
            Self::VideoResolutionNotSupported => String::from("VideoResolutionNotSupported"),
            Self::UnknownVideoStreamInfo => String::from("UnknownVideoStreamInfo"),
            Self::UnknownAudioStreamInfo => String::from("UnknownAudioStreamInfo"),
            Self::AudioProfileNotSupported => String::from("AudioProfileNotSupported"),
            Self::AudioSampleRateNotSupported => String::from("AudioSampleRateNotSupported"),
            Self::AnamorphicVideoNotSupported => String::from("AnamorphicVideoNotSupported"),
            Self::InterlacedVideoNotSupported => String::from("InterlacedVideoNotSupported"),
            Self::SecondaryAudioNotSupported => String::from("SecondaryAudioNotSupported"),
            Self::RefFramesNotSupported => String::from("RefFramesNotSupported"),
            Self::VideoBitDepthNotSupported => String::from("VideoBitDepthNotSupported"),
            Self::VideoBitrateNotSupported => String::from("VideoBitrateNotSupported"),
            Self::VideoFramerateNotSupported => String::from("VideoFramerateNotSupported"),
            Self::VideoLevelNotSupported => String::from("VideoLevelNotSupported"),
            Self::VideoProfileNotSupported => String::from("VideoProfileNotSupported"),
            Self::AudioBitDepthNotSupported => String::from("AudioBitDepthNotSupported"),
            Self::SubtitleCodecNotSupported => String::from("SubtitleCodecNotSupported"),
            Self::DirectPlayError => String::from("DirectPlayError"),
        }
    }
}
