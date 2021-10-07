//! Audio stream operations
use bytes::Bytes;
use derive_builder::Builder;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;

use crate::auth::AuthHeader;
use crate::error::SeascapeError::InvalidContent;
use crate::http::{DataContentType, HttpClient};

use super::{ClientResult, Jellyfin};

impl Jellyfin {
    /// Gets an audio stream. This stream only supports direct play (no transcoding).
    pub async fn audio_stream(&self, stream_query: &AudioStreamQuery) -> ClientResult<Bytes> {
        let url = format!(
            "{}/{}/{}/{}",
            self.base_url, "audio", stream_query.item_id, "stream"
        );

        let is_static = stream_query.is_static.map(|x| x.to_string());
        let stream_params = stream_query.params.as_ref().map(|x| x.join(","));
        let segment_length = stream_query.segment_length.map(|x| x.to_string());
        let min_segments = stream_query.min_segments.map(|x| x.to_string());
        let enable_auto_stream_copy = stream_query.enable_auto_stream_copy.map(|x| x.to_string());
        let allow_video_stream_copy = stream_query.allow_video_stream_copy.map(|x| x.to_string());
        let allow_audio_stream_copy = stream_query.allow_audio_stream_copy.map(|x| x.to_string());
        let break_on_non_key_frames = stream_query.break_on_non_key_frames.map(|x| x.to_string());
        let audio_sample_rate = stream_query.audio_sample_rate.map(|x| x.to_string());
        let max_audio_bit_depth = stream_query.max_audio_bit_depth.map(|x| x.to_string());
        let audio_bit_rate = stream_query.audio_bit_rate.map(|x| x.to_string());
        let audio_channels = stream_query.audio_channels.map(|x| x.to_string());
        let max_audio_channels = stream_query.max_audio_channels.map(|x| x.to_string());
        let framerate = stream_query.framerate.map(|x| x.to_string());
        let max_framerate = stream_query.framerate.map(|x| x.to_string());
        let copy_timestamps = stream_query.copy_timestamps.map(|x| x.to_string());
        let start_time_ticks = stream_query.start_time_ticks.map(|x| x.to_string());
        let width = stream_query.width.map(|x| x.to_string());
        let height = stream_query.height.map(|x| x.to_string());
        let video_bit_rate = stream_query.video_bit_rate.map(|x| x.to_string());
        let subtitle_stream_index = stream_query.subtitle_stream_index.map(|x| x.to_string());
        let subtitle_method = stream_query.subtitle_method.as_ref().map(|x| x.to_string());
        let max_ref_frames = stream_query.max_ref_frames.map(|x| x.to_string());
        let max_video_bit_depth = stream_query.max_video_bit_depth.map(|x| x.to_string());
        let require_avc = stream_query.require_avc.map(|x| x.to_string());
        let deinterlace = stream_query.deinterlace.map(|x| x.to_string());
        let require_non_anamorphic = stream_query.require_non_anamorphic.map(|x| x.to_string());
        let transcoding_max_audio_channels = stream_query
            .transcoding_max_audio_channels
            .map(|x| x.to_string());
        let cpu_core_limit = stream_query.cpu_core_limit.map(|x| x.to_string());
        let enable_mpegts_m2ts_mode = stream_query.enable_mpegts_m2ts_mode.map(|x| x.to_string());
        let audio_stream_index = stream_query.audio_stream_index.map(|x| x.to_string());
        let video_stream_index = stream_query.video_stream_index.map(|x| x.to_string());
        let context = stream_query.context.as_ref().map(|x| x.to_string());
        let stream_options = stream_query
            .stream_options
            .as_ref()
            .map(|x| x.iter().map(|x| format!("{}={}", x.0, x.1)).join("&"));

        let params = build_map!(
            "container": stream_query.container.as_ref(),
            optional "static": is_static.as_deref(),
            optional "params": stream_params.as_deref(),
            optional "tag": stream_query.tag.as_deref(),
            optional "deviceProfileId": stream_query.device_profile_id.as_deref(),
            optional "playSessionId": stream_query.play_session_id.as_deref(),
            optional "segmentContainer": stream_query.segment_container.as_deref(),
            optional "segmentLength": segment_length.as_deref(),
            optional "minSegments": min_segments.as_deref(),
            optional "mediaSourceId": stream_query.media_source_id.as_deref(),
            optional "deviceId": stream_query.device_profile_id.as_deref(),
            optional "audioCodec": stream_query.audio_codec.as_deref(),
            optional "enableAutoStreamCopy": enable_auto_stream_copy.as_deref(),
            optional "allowVideoStreamCopy": allow_video_stream_copy.as_deref(),
            optional "allowAudioStreamCopy": allow_audio_stream_copy.as_deref(),
            optional "breakOnNonKeyFrames": break_on_non_key_frames.as_deref(),
            optional "audioSampleRate": audio_sample_rate.as_deref(),
            optional "maxAudioBitDepth": max_audio_bit_depth.as_deref(),
            optional "audioBitrate": audio_bit_rate.as_deref(),
            optional "audioChannels": audio_channels.as_deref(),
            optional "maxAudioChannels": max_audio_channels.as_deref(),
            optional "profile": stream_query.profile.as_deref(),
            optional "level": stream_query.level.as_deref(),
            optional "framerate": framerate.as_deref(),
            optional "max_framerate": max_framerate.as_deref(),
            optional "copyTimestamps": copy_timestamps.as_deref(),
            optional "startTimeTicks": start_time_ticks.as_deref(),
            optional "width": width.as_deref(),
            optional "height": height.as_deref(),
            optional "videoBitRate": video_bit_rate.as_deref(),
            optional "subtitleStreamIndex": subtitle_stream_index.as_deref(),
            optional "subtitleMethod": subtitle_method.as_deref(),
            optional "maxRefFrames": max_ref_frames.as_deref(),
            optional "maxVideoBithDepth": max_video_bit_depth.as_deref(),
            optional "requireAvc": require_avc.as_deref(),
            optional "deinterlace": deinterlace.as_deref(),
            optional "requireNonAnamorphic": require_non_anamorphic.as_deref(),
            optional "transcodingMaxAudioChannels": transcoding_max_audio_channels.as_deref(),
            optional "cpuCoreLimit": cpu_core_limit.as_deref(),
            optional "liveStreamId": stream_query.live_stream_id.as_deref(),
            optional "enableMpegtsM2tsMode": enable_mpegts_m2ts_mode.as_deref(),
            optional "videoCodec": stream_query.video_codec.as_deref(),
            optional "transcodeReasons": transcoding_max_audio_channels.as_deref(),
            optional "audioStreamIndex": audio_stream_index.as_deref(),
            optional "videoStreamIndex": video_stream_index.as_deref(),
            optional "context": context.as_deref(),
            optional "streamOptions": stream_options.as_deref(),
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
pub struct AudioStreamQuery {
    item_id: String,
    container: String,
    is_static: Option<bool>,
    params: Option<Vec<String>>,
    tag: Option<String>,
    device_profile_id: Option<String>,
    play_session_id: Option<String>,
    segment_container: Option<String>,
    segment_length: Option<i32>,
    min_segments: Option<i32>,
    media_source_id: Option<String>,
    device_id: Option<String>,
    audio_codec: Option<String>,
    enable_auto_stream_copy: Option<bool>,
    allow_video_stream_copy: Option<bool>,
    allow_audio_stream_copy: Option<bool>,
    break_on_non_key_frames: Option<bool>,
    audio_sample_rate: Option<i32>,
    max_audio_bit_depth: Option<i32>,
    audio_bit_rate: Option<i32>,
    audio_channels: Option<i32>,
    max_audio_channels: Option<i32>,
    profile: Option<String>,
    level: Option<String>,
    framerate: Option<i64>,
    max_framerate: Option<i64>,
    copy_timestamps: Option<bool>,
    start_time_ticks: Option<i64>,
    width: Option<i32>,
    height: Option<i32>,
    video_bit_rate: Option<i32>,
    subtitle_stream_index: Option<i32>,
    subtitle_method: Option<AudioQuerySubtitleMethod>,
    max_ref_frames: Option<i32>,
    max_video_bit_depth: Option<i32>,
    require_avc: Option<bool>,
    deinterlace: Option<bool>,
    require_non_anamorphic: Option<bool>,
    transcoding_max_audio_channels: Option<i32>,
    cpu_core_limit: Option<i32>,
    live_stream_id: Option<String>,
    enable_mpegts_m2ts_mode: Option<bool>,
    video_codec: Option<String>,
    transcode_reasons: Option<String>,
    audio_stream_index: Option<i32>,
    video_stream_index: Option<i32>,
    context: Option<AudioQueryContext>,
    stream_options: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug)]
pub enum AudioQueryContext {
    Streaming,
    Static,
}

impl Display for AudioQueryContext {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub enum AudioQuerySubtitleMethod {
    Encode,
    Embed,
    External,
    Hls,
}

impl Display for AudioQuerySubtitleMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
