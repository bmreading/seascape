//! Universal audio streaming

use bytes::Bytes;
use derive_builder::Builder;

use crate::http::{DataContentType, HttpClient};
use crate::{auth::AuthHeader, error::SeascapeError::InvalidContent};

use super::{ClientResult, Jellyfin};

impl Jellyfin {
    /// Gets an audio stream.
    pub async fn universal_audio_stream(
        &self,
        stream_query: &UniversalAudioStreamQuery,
    ) -> ClientResult<Bytes> {
        let url = format!(
            "{}/{}/{}/{}",
            self.base_url, "audio", stream_query.item_id, "universal"
        );

        let container = stream_query.container.as_ref().map(|x| x.join(","));
        let max_audio_channels = stream_query
            .max_audio_channels
            .as_ref()
            .map(|x| x.to_string());
        let transcoding_audio_channels = stream_query
            .transcoding_audio_channels
            .as_ref()
            .map(|x| x.to_string());
        let max_streaming_bitrate = stream_query
            .max_streaming_bitrate
            .as_ref()
            .map(|x| x.to_string());
        let audio_bitrate = stream_query.audio_bitrate.as_ref().map(|x| x.to_string());
        let start_time_ticks = stream_query
            .start_time_ticks
            .as_ref()
            .map(|x| x.to_string());
        let max_audio_sample_rate = stream_query
            .max_audio_sample_rate
            .as_ref()
            .map(|x| x.to_string());
        let max_audio_bit_depth = stream_query
            .max_audio_bit_depth
            .as_ref()
            .map(|x| x.to_string());
        let enable_remote_media = stream_query
            .enable_remote_media
            .as_ref()
            .map(|x| x.to_string());
        let break_on_non_key_frames = stream_query
            .break_on_non_key_frames
            .as_ref()
            .map(|x| x.to_string());
        let enable_redirection = stream_query
            .enable_redirection
            .as_ref()
            .map(|x| x.to_string());

        let params = build_map!(
            optional "container": container.as_ref(),
            optional "mediaSourceId": stream_query.media_source_id.as_deref(),
            optional "deviceId": stream_query.device_id.as_deref(),
            optional "userId": stream_query.user_id.as_deref(),
            optional "audioCodec": stream_query.audio_codec.as_deref(),
            optional "maxAudioChannels": max_audio_channels.as_deref(),
            optional "transcodingAudioChannels": transcoding_audio_channels.as_deref(),
            optional "maxStreamingBitrate": max_streaming_bitrate.as_deref(),
            optional "audioBitrate": audio_bitrate.as_deref(),
            optional "startTimeTicks": start_time_ticks.as_deref(),
            optional "transcodingContainer": stream_query.transcoding_container.as_deref(),
            optional "transcodingProtocol": stream_query.transcoding_protocol.as_deref(),
            optional "maxAudioSampleRate": max_audio_sample_rate.as_deref(),
            optional "maxAudioBitDepth": max_audio_bit_depth.as_deref(),
            optional "enableRemoteMedia": enable_remote_media.as_deref(),
            optional "breakOnNonKeyFrames": break_on_non_key_frames.as_deref(),
            optional "enableRedirection": enable_redirection.as_deref(),
        );

        let request = http::request::Request::builder()
            .uri(url)
            .method("GET")
            .header(
                self.auth_header_type.as_ref().unwrap().header_key_name(),
                self.auth_header_type.as_ref().unwrap().header_value(),
            )
            .body(None)?;

        let response = self.http_client_type.send(&request, Some(&params)).await?;

        match response.body() {
            DataContentType::BinaryContent(c) => Ok(c.to_owned()),
            DataContentType::TextContent(_) => Err(InvalidContent),
            DataContentType::NoContent => Err(InvalidContent),
        }
    }
}

#[derive(Builder, Clone, Debug, Default)]
#[builder(setter(into, strip_option), default)]
pub struct UniversalAudioStreamQuery {
    item_id: String,
    container: Option<Vec<String>>,
    media_source_id: Option<String>,
    device_id: Option<String>,
    user_id: Option<String>,
    audio_codec: Option<String>,
    max_audio_channels: Option<i32>,
    transcoding_audio_channels: Option<i32>,
    max_streaming_bitrate: Option<i32>,
    audio_bitrate: Option<i32>,
    start_time_ticks: Option<i64>,
    transcoding_container: Option<String>,
    transcoding_protocol: Option<String>,
    max_audio_sample_rate: Option<i32>,
    max_audio_bit_depth: Option<i32>,
    enable_remote_media: Option<bool>,
    break_on_non_key_frames: Option<bool>,
    enable_redirection: Option<bool>,
}
