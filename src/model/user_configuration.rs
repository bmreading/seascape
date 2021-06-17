use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserConfiguration {
    /// Gets or sets the audio language preference.
    #[serde(
        rename = "AudioLanguagePreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_language_preference: Option<String>,
    /// Gets or sets a value indicating whether [play default audio track].
    #[serde(
        rename = "PlayDefaultAudioTrack",
        skip_serializing_if = "Option::is_none"
    )]
    pub play_default_audio_track: Option<bool>,
    /// Gets or sets the subtitle language preference.
    #[serde(
        rename = "SubtitleLanguagePreference",
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_language_preference: Option<String>,
    #[serde(
        rename = "DisplayMissingEpisodes",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_missing_episodes: Option<bool>,
    #[serde(rename = "GroupedFolders", skip_serializing_if = "Option::is_none")]
    pub grouped_folders: Option<Vec<String>>,
    /// An enum representing a subtitle playback mode.
    #[serde(rename = "SubtitleMode", skip_serializing_if = "Option::is_none")]
    pub subtitle_mode: Option<Box<super::subtitle_playback_mode::SubtitlePlaybackMode>>,
    #[serde(
        rename = "DisplayCollectionsView",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_collections_view: Option<bool>,
    #[serde(
        rename = "EnableLocalPassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_local_password: Option<bool>,
    #[serde(rename = "OrderedViews", skip_serializing_if = "Option::is_none")]
    pub ordered_views: Option<Vec<String>>,
    #[serde(
        rename = "LatestItemsExcludes",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_items_excludes: Option<Vec<String>>,
    #[serde(rename = "MyMediaExcludes", skip_serializing_if = "Option::is_none")]
    pub my_media_excludes: Option<Vec<String>>,
    #[serde(rename = "HidePlayedInLatest", skip_serializing_if = "Option::is_none")]
    pub hide_played_in_latest: Option<bool>,
    #[serde(
        rename = "RememberAudioSelections",
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_audio_selections: Option<bool>,
    #[serde(
        rename = "RememberSubtitleSelections",
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_subtitle_selections: Option<bool>,
    #[serde(
        rename = "EnableNextEpisodeAutoPlay",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_next_episode_auto_play: Option<bool>,
}

impl UserConfiguration {
    /// Class UserConfiguration.
    pub fn new() -> UserConfiguration {
        UserConfiguration {
            audio_language_preference: None,
            play_default_audio_track: None,
            subtitle_language_preference: None,
            display_missing_episodes: None,
            grouped_folders: None,
            subtitle_mode: None,
            display_collections_view: None,
            enable_local_password: None,
            ordered_views: None,
            latest_items_excludes: None,
            my_media_excludes: None,
            hide_played_in_latest: None,
            remember_audio_selections: None,
            remember_subtitle_selections: None,
            enable_next_episode_auto_play: None,
        }
    }
}
