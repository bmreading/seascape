use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProfileConditionValue {
    #[serde(rename = "AudioChannels")]
    AudioChannels,
    #[serde(rename = "AudioBitrate")]
    AudioBitrate,
    #[serde(rename = "AudioProfile")]
    AudioProfile,
    #[serde(rename = "Width")]
    Width,
    #[serde(rename = "Height")]
    Height,
    #[serde(rename = "Has64BitOffsets")]
    Has64BitOffsets,
    #[serde(rename = "PacketLength")]
    PacketLength,
    #[serde(rename = "VideoBitDepth")]
    VideoBitDepth,
    #[serde(rename = "VideoBitrate")]
    VideoBitrate,
    #[serde(rename = "VideoFramerate")]
    VideoFramerate,
    #[serde(rename = "VideoLevel")]
    VideoLevel,
    #[serde(rename = "VideoProfile")]
    VideoProfile,
    #[serde(rename = "VideoTimestamp")]
    VideoTimestamp,
    #[serde(rename = "IsAnamorphic")]
    IsAnamorphic,
    #[serde(rename = "RefFrames")]
    RefFrames,
    #[serde(rename = "NumAudioStreams")]
    NumAudioStreams,
    #[serde(rename = "NumVideoStreams")]
    NumVideoStreams,
    #[serde(rename = "IsSecondaryAudio")]
    IsSecondaryAudio,
    #[serde(rename = "VideoCodecTag")]
    VideoCodecTag,
    #[serde(rename = "IsAvc")]
    IsAvc,
    #[serde(rename = "IsInterlaced")]
    IsInterlaced,
    #[serde(rename = "AudioSampleRate")]
    AudioSampleRate,
    #[serde(rename = "AudioBitDepth")]
    AudioBitDepth,
}

impl ToString for ProfileConditionValue {
    fn to_string(&self) -> String {
        match self {
            Self::AudioChannels => String::from("AudioChannels"),
            Self::AudioBitrate => String::from("AudioBitrate"),
            Self::AudioProfile => String::from("AudioProfile"),
            Self::Width => String::from("Width"),
            Self::Height => String::from("Height"),
            Self::Has64BitOffsets => String::from("Has64BitOffsets"),
            Self::PacketLength => String::from("PacketLength"),
            Self::VideoBitDepth => String::from("VideoBitDepth"),
            Self::VideoBitrate => String::from("VideoBitrate"),
            Self::VideoFramerate => String::from("VideoFramerate"),
            Self::VideoLevel => String::from("VideoLevel"),
            Self::VideoProfile => String::from("VideoProfile"),
            Self::VideoTimestamp => String::from("VideoTimestamp"),
            Self::IsAnamorphic => String::from("IsAnamorphic"),
            Self::RefFrames => String::from("RefFrames"),
            Self::NumAudioStreams => String::from("NumAudioStreams"),
            Self::NumVideoStreams => String::from("NumVideoStreams"),
            Self::IsSecondaryAudio => String::from("IsSecondaryAudio"),
            Self::VideoCodecTag => String::from("VideoCodecTag"),
            Self::IsAvc => String::from("IsAvc"),
            Self::IsInterlaced => String::from("IsInterlaced"),
            Self::AudioSampleRate => String::from("AudioSampleRate"),
            Self::AudioBitDepth => String::from("AudioBitDepth"),
        }
    }
}
