use serde::{Deserialize, Serialize};
/// This holds information about a BaseItem in a format that is convenient for the server API

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemDto {
    /// The name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginalTitle", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    /// The server identifier.
    #[serde(rename = "ServerId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// The id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The etag.
    #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The type of the source.
    #[serde(rename = "SourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The playlist item identifier.
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
    /// The date created.
    #[serde(rename = "DateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateLastMediaAdded", skip_serializing_if = "Option::is_none")]
    pub date_last_media_added: Option<String>,
    #[serde(rename = "ExtraType", skip_serializing_if = "Option::is_none")]
    pub extra_type: Option<String>,
    #[serde(
        rename = "AirsBeforeSeasonNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_before_season_number: Option<i32>,
    #[serde(
        rename = "AirsAfterSeasonNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_after_season_number: Option<i32>,
    #[serde(
        rename = "AirsBeforeEpisodeNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub airs_before_episode_number: Option<i32>,
    #[serde(rename = "CanDelete", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "CanDownload", skip_serializing_if = "Option::is_none")]
    pub can_download: Option<bool>,
    #[serde(rename = "HasSubtitles", skip_serializing_if = "Option::is_none")]
    pub has_subtitles: Option<bool>,
    #[serde(
        rename = "PreferredMetadataLanguage",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[serde(
        rename = "PreferredMetadataCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_country_code: Option<String>,
    /// A value indicating whether [supports synchronize].
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// The name of the sort.
    #[serde(rename = "SortName", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    #[serde(rename = "ForcedSortName", skip_serializing_if = "Option::is_none")]
    pub forced_sort_name: Option<String>,
    /// The video 3D format.
    #[serde(rename = "Video3DFormat", skip_serializing_if = "Option::is_none")]
    pub video3_d_format: Option<Box<crate::model::Video3DFormat>>,
    /// The premiere date.
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    /// The external urls.
    #[serde(rename = "ExternalUrls", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Vec<crate::model::ExternalUrl>>,
    /// The media versions.
    #[serde(rename = "MediaSources", skip_serializing_if = "Option::is_none")]
    pub media_sources: Option<Vec<crate::model::MediaSourceInfo>>,
    /// The critic rating.
    #[serde(rename = "CriticRating", skip_serializing_if = "Option::is_none")]
    pub critic_rating: Option<f32>,
    #[serde(
        rename = "ProductionLocations",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_locations: Option<Vec<String>>,
    /// The path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "EnableMediaSourceDisplay",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_media_source_display: Option<bool>,
    /// The official rating.
    #[serde(rename = "OfficialRating", skip_serializing_if = "Option::is_none")]
    pub official_rating: Option<String>,
    /// The custom rating.
    #[serde(rename = "CustomRating", skip_serializing_if = "Option::is_none")]
    pub custom_rating: Option<String>,
    /// The channel identifier.
    #[serde(rename = "ChannelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// The overview.
    #[serde(rename = "Overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    /// The taglines.
    #[serde(rename = "Taglines", skip_serializing_if = "Option::is_none")]
    pub taglines: Option<Vec<String>>,
    /// The genres.
    #[serde(rename = "Genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// The community rating.
    #[serde(rename = "CommunityRating", skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<f32>,
    /// The cumulative run time ticks.
    #[serde(
        rename = "CumulativeRunTimeTicks",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_run_time_ticks: Option<i64>,
    /// The run time ticks.
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    /// The play access.
    #[serde(rename = "PlayAccess", skip_serializing_if = "Option::is_none")]
    pub play_access: Option<Box<crate::model::PlayAccess>>,
    /// The aspect ratio.
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// The production year.
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    /// A value indicating whether this instance is place holder.
    #[serde(rename = "IsPlaceHolder", skip_serializing_if = "Option::is_none")]
    pub is_place_holder: Option<bool>,
    /// The number.
    #[serde(rename = "Number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "ChannelNumber", skip_serializing_if = "Option::is_none")]
    pub channel_number: Option<String>,
    /// The index number.
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    /// The index number end.
    #[serde(rename = "IndexNumberEnd", skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<i32>,
    /// The parent index number.
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    /// The trailer urls.
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option::is_none")]
    pub remote_trailers: Option<Vec<crate::model::MediaUrl>>,
    /// The provider ids.
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<::std::collections::HashMap<String, String>>,
    /// A value indicating whether this instance is HD.
    #[serde(rename = "IsHD", skip_serializing_if = "Option::is_none")]
    pub is_hd: Option<bool>,
    /// A value indicating whether this instance is folder.
    #[serde(rename = "IsFolder", skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    /// The parent id.
    #[serde(rename = "ParentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// The people.
    #[serde(rename = "People", skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<crate::model::BaseItemPerson>>,
    /// The studios.
    #[serde(rename = "Studios", skip_serializing_if = "Option::is_none")]
    pub studios: Option<Vec<crate::model::NameGuidPair>>,
    #[serde(rename = "GenreItems", skip_serializing_if = "Option::is_none")]
    pub genre_items: Option<Vec<crate::model::NameGuidPair>>,
    /// If the item does not have a logo, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentLogoItemId", skip_serializing_if = "Option::is_none")]
    pub parent_logo_item_id: Option<String>,
    /// If the item does not have any backdrops, this will hold the Id of the Parent that has one.
    #[serde(
        rename = "ParentBackdropItemId",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_item_id: Option<String>,
    /// The parent backdrop image tags.
    #[serde(
        rename = "ParentBackdropImageTags",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_image_tags: Option<Vec<String>>,
    /// The local trailer count.
    #[serde(rename = "LocalTrailerCount", skip_serializing_if = "Option::is_none")]
    pub local_trailer_count: Option<i32>,
    /// User data for this item based on the user it's being requested for.
    #[serde(rename = "UserData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Box<crate::model::UserItemDataDto>>,
    /// The recursive item count.
    #[serde(rename = "RecursiveItemCount", skip_serializing_if = "Option::is_none")]
    pub recursive_item_count: Option<i32>,
    /// The child count.
    #[serde(rename = "ChildCount", skip_serializing_if = "Option::is_none")]
    pub child_count: Option<i32>,
    /// The name of the series.
    #[serde(rename = "SeriesName", skip_serializing_if = "Option::is_none")]
    pub series_name: Option<String>,
    /// The series id.
    #[serde(rename = "SeriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<String>,
    /// The season identifier.
    #[serde(rename = "SeasonId", skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    /// The special feature count.
    #[serde(
        rename = "SpecialFeatureCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub special_feature_count: Option<i32>,
    /// The display preferences id.
    #[serde(
        rename = "DisplayPreferencesId",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_preferences_id: Option<String>,
    /// The status.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The air time.
    #[serde(rename = "AirTime", skip_serializing_if = "Option::is_none")]
    pub air_time: Option<String>,
    /// The air days.
    #[serde(rename = "AirDays", skip_serializing_if = "Option::is_none")]
    pub air_days: Option<Vec<crate::model::DayOfWeek>>,
    /// The tags.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The primary image aspect ratio, after image enhancements.
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
    /// The artists.
    #[serde(rename = "Artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
    /// The artist items.
    #[serde(rename = "ArtistItems", skip_serializing_if = "Option::is_none")]
    pub artist_items: Option<Vec<crate::model::NameGuidPair>>,
    /// The album.
    #[serde(rename = "Album", skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    /// The type of the collection.
    #[serde(rename = "CollectionType", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// The display order.
    #[serde(rename = "DisplayOrder", skip_serializing_if = "Option::is_none")]
    pub display_order: Option<String>,
    /// The album id.
    #[serde(rename = "AlbumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    /// The album image tag.
    #[serde(
        rename = "AlbumPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub album_primary_image_tag: Option<String>,
    /// The series primary image tag.
    #[serde(
        rename = "SeriesPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub series_primary_image_tag: Option<String>,
    /// The album artist.
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<String>,
    /// The album artists.
    #[serde(rename = "AlbumArtists", skip_serializing_if = "Option::is_none")]
    pub album_artists: Option<Vec<crate::model::NameGuidPair>>,
    /// The name of the season.
    #[serde(rename = "SeasonName", skip_serializing_if = "Option::is_none")]
    pub season_name: Option<String>,
    /// The media streams.
    #[serde(rename = "MediaStreams", skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<crate::model::MediaStream>>,
    /// The type of the video.
    #[serde(rename = "VideoType", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<Box<crate::model::VideoType>>,
    /// The part count.
    #[serde(rename = "PartCount", skip_serializing_if = "Option::is_none")]
    pub part_count: Option<i32>,
    #[serde(rename = "MediaSourceCount", skip_serializing_if = "Option::is_none")]
    pub media_source_count: Option<i32>,
    /// The image tags.
    #[serde(rename = "ImageTags", skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<::std::collections::HashMap<String, String>>,
    /// The backdrop image tags.
    #[serde(rename = "BackdropImageTags", skip_serializing_if = "Option::is_none")]
    pub backdrop_image_tags: Option<Vec<String>>,
    /// The screenshot image tags.
    #[serde(
        rename = "ScreenshotImageTags",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_image_tags: Option<Vec<String>>,
    /// The parent logo image tag.
    #[serde(rename = "ParentLogoImageTag", skip_serializing_if = "Option::is_none")]
    pub parent_logo_image_tag: Option<String>,
    /// If the item does not have a art, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentArtItemId", skip_serializing_if = "Option::is_none")]
    pub parent_art_item_id: Option<String>,
    /// The parent art image tag.
    #[serde(rename = "ParentArtImageTag", skip_serializing_if = "Option::is_none")]
    pub parent_art_image_tag: Option<String>,
    /// The series thumb image tag.
    #[serde(
        rename = "SeriesThumbImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub series_thumb_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Box<crate::model::BaseItemDtoImageBlurHashes>>,
    /// The series studio.
    #[serde(rename = "SeriesStudio", skip_serializing_if = "Option::is_none")]
    pub series_studio: Option<String>,
    /// The parent thumb item id.
    #[serde(rename = "ParentThumbItemId", skip_serializing_if = "Option::is_none")]
    pub parent_thumb_item_id: Option<String>,
    /// The parent thumb image tag.
    #[serde(
        rename = "ParentThumbImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_thumb_image_tag: Option<String>,
    /// The parent primary image item identifier.
    #[serde(
        rename = "ParentPrimaryImageItemId",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_item_id: Option<String>,
    /// The parent primary image tag.
    #[serde(
        rename = "ParentPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_tag: Option<String>,
    /// The chapters.
    #[serde(rename = "Chapters", skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<crate::model::ChapterInfo>>,
    /// The type of the location.
    #[serde(rename = "LocationType", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<Box<crate::model::LocationType>>,
    /// The type of the iso.
    #[serde(rename = "IsoType", skip_serializing_if = "Option::is_none")]
    pub iso_type: Option<Box<crate::model::IsoType>>,
    /// The type of the media.
    #[serde(rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The end date.
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// The locked fields.
    #[serde(rename = "LockedFields", skip_serializing_if = "Option::is_none")]
    pub locked_fields: Option<Vec<crate::model::MetadataField>>,
    /// The trailer count.
    #[serde(rename = "TrailerCount", skip_serializing_if = "Option::is_none")]
    pub trailer_count: Option<i32>,
    /// The movie count.
    #[serde(rename = "MovieCount", skip_serializing_if = "Option::is_none")]
    pub movie_count: Option<i32>,
    /// The series count.
    #[serde(rename = "SeriesCount", skip_serializing_if = "Option::is_none")]
    pub series_count: Option<i32>,
    #[serde(rename = "ProgramCount", skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    /// The episode count.
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    /// The song count.
    #[serde(rename = "SongCount", skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    /// The album count.
    #[serde(rename = "AlbumCount", skip_serializing_if = "Option::is_none")]
    pub album_count: Option<i32>,
    #[serde(rename = "ArtistCount", skip_serializing_if = "Option::is_none")]
    pub artist_count: Option<i32>,
    /// The music video count.
    #[serde(rename = "MusicVideoCount", skip_serializing_if = "Option::is_none")]
    pub music_video_count: Option<i32>,
    /// A value indicating whether [enable internet providers].
    #[serde(rename = "LockData", skip_serializing_if = "Option::is_none")]
    pub lock_data: Option<bool>,
    #[serde(rename = "Width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "Height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "CameraMake", skip_serializing_if = "Option::is_none")]
    pub camera_make: Option<String>,
    #[serde(rename = "CameraModel", skip_serializing_if = "Option::is_none")]
    pub camera_model: Option<String>,
    #[serde(rename = "Software", skip_serializing_if = "Option::is_none")]
    pub software: Option<String>,
    #[serde(rename = "ExposureTime", skip_serializing_if = "Option::is_none")]
    pub exposure_time: Option<f64>,
    #[serde(rename = "FocalLength", skip_serializing_if = "Option::is_none")]
    pub focal_length: Option<f64>,
    #[serde(rename = "ImageOrientation", skip_serializing_if = "Option::is_none")]
    pub image_orientation: Option<Box<crate::model::ImageOrientation>>,
    #[serde(rename = "Aperture", skip_serializing_if = "Option::is_none")]
    pub aperture: Option<f64>,
    #[serde(rename = "ShutterSpeed", skip_serializing_if = "Option::is_none")]
    pub shutter_speed: Option<f64>,
    #[serde(rename = "Latitude", skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude", skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(rename = "Altitude", skip_serializing_if = "Option::is_none")]
    pub altitude: Option<f64>,
    #[serde(rename = "IsoSpeedRating", skip_serializing_if = "Option::is_none")]
    pub iso_speed_rating: Option<i32>,
    /// The series timer identifier.
    #[serde(rename = "SeriesTimerId", skip_serializing_if = "Option::is_none")]
    pub series_timer_id: Option<String>,
    /// The program identifier.
    #[serde(rename = "ProgramId", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    /// The channel primary image tag.
    #[serde(
        rename = "ChannelPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    /// The start date of the recording, in UTC.
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The completion percentage.
    #[serde(
        rename = "CompletionPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_percentage: Option<f64>,
    /// A value indicating whether this instance is repeat.
    #[serde(rename = "IsRepeat", skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<bool>,
    /// The episode title.
    #[serde(rename = "EpisodeTitle", skip_serializing_if = "Option::is_none")]
    pub episode_title: Option<String>,
    /// The type of the channel.
    #[serde(rename = "ChannelType", skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<Box<crate::model::ChannelType>>,
    /// The audio.
    #[serde(rename = "Audio", skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<crate::model::ProgramAudio>>,
    /// A value indicating whether this instance is movie.
    #[serde(rename = "IsMovie", skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<bool>,
    /// A value indicating whether this instance is sports.
    #[serde(rename = "IsSports", skip_serializing_if = "Option::is_none")]
    pub is_sports: Option<bool>,
    /// A value indicating whether this instance is series.
    #[serde(rename = "IsSeries", skip_serializing_if = "Option::is_none")]
    pub is_series: Option<bool>,
    /// A value indicating whether this instance is live.
    #[serde(rename = "IsLive", skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    /// A value indicating whether this instance is news.
    #[serde(rename = "IsNews", skip_serializing_if = "Option::is_none")]
    pub is_news: Option<bool>,
    /// A value indicating whether this instance is kids.
    #[serde(rename = "IsKids", skip_serializing_if = "Option::is_none")]
    pub is_kids: Option<bool>,
    /// A value indicating whether this instance is premiere.
    #[serde(rename = "IsPremiere", skip_serializing_if = "Option::is_none")]
    pub is_premiere: Option<bool>,
    /// The timer identifier.
    #[serde(rename = "TimerId", skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
    /// The current program.
    #[serde(rename = "CurrentProgram", skip_serializing_if = "Option::is_none")]
    pub current_program: Option<Box<crate::model::BaseItemDto>>,
}

impl BaseItemDto {
    /// Returns a new BaseItemDto instance
    pub fn new() -> BaseItemDto {
        BaseItemDto {
            name: None,
            original_title: None,
            server_id: None,
            id: None,
            etag: None,
            source_type: None,
            playlist_item_id: None,
            date_created: None,
            date_last_media_added: None,
            extra_type: None,
            airs_before_season_number: None,
            airs_after_season_number: None,
            airs_before_episode_number: None,
            can_delete: None,
            can_download: None,
            has_subtitles: None,
            preferred_metadata_language: None,
            preferred_metadata_country_code: None,
            supports_sync: None,
            container: None,
            sort_name: None,
            forced_sort_name: None,
            video3_d_format: None,
            premiere_date: None,
            external_urls: None,
            media_sources: None,
            critic_rating: None,
            production_locations: None,
            path: None,
            enable_media_source_display: None,
            official_rating: None,
            custom_rating: None,
            channel_id: None,
            channel_name: None,
            overview: None,
            taglines: None,
            genres: None,
            community_rating: None,
            cumulative_run_time_ticks: None,
            run_time_ticks: None,
            play_access: None,
            aspect_ratio: None,
            production_year: None,
            is_place_holder: None,
            number: None,
            channel_number: None,
            index_number: None,
            index_number_end: None,
            parent_index_number: None,
            remote_trailers: None,
            provider_ids: None,
            is_hd: None,
            is_folder: None,
            parent_id: None,
            _type: None,
            people: None,
            studios: None,
            genre_items: None,
            parent_logo_item_id: None,
            parent_backdrop_item_id: None,
            parent_backdrop_image_tags: None,
            local_trailer_count: None,
            user_data: None,
            recursive_item_count: None,
            child_count: None,
            series_name: None,
            series_id: None,
            season_id: None,
            special_feature_count: None,
            display_preferences_id: None,
            status: None,
            air_time: None,
            air_days: None,
            tags: None,
            primary_image_aspect_ratio: None,
            artists: None,
            artist_items: None,
            album: None,
            collection_type: None,
            display_order: None,
            album_id: None,
            album_primary_image_tag: None,
            series_primary_image_tag: None,
            album_artist: None,
            album_artists: None,
            season_name: None,
            media_streams: None,
            video_type: None,
            part_count: None,
            media_source_count: None,
            image_tags: None,
            backdrop_image_tags: None,
            screenshot_image_tags: None,
            parent_logo_image_tag: None,
            parent_art_item_id: None,
            parent_art_image_tag: None,
            series_thumb_image_tag: None,
            image_blur_hashes: None,
            series_studio: None,
            parent_thumb_item_id: None,
            parent_thumb_image_tag: None,
            parent_primary_image_item_id: None,
            parent_primary_image_tag: None,
            chapters: None,
            location_type: None,
            iso_type: None,
            media_type: None,
            end_date: None,
            locked_fields: None,
            trailer_count: None,
            movie_count: None,
            series_count: None,
            program_count: None,
            episode_count: None,
            song_count: None,
            album_count: None,
            artist_count: None,
            music_video_count: None,
            lock_data: None,
            width: None,
            height: None,
            camera_make: None,
            camera_model: None,
            software: None,
            exposure_time: None,
            focal_length: None,
            image_orientation: None,
            aperture: None,
            shutter_speed: None,
            latitude: None,
            longitude: None,
            altitude: None,
            iso_speed_rating: None,
            series_timer_id: None,
            program_id: None,
            channel_primary_image_tag: None,
            start_date: None,
            completion_percentage: None,
            is_repeat: None,
            episode_title: None,
            channel_type: None,
            audio: None,
            is_movie: None,
            is_sports: None,
            is_series: None,
            is_live: None,
            is_news: None,
            is_kids: None,
            is_premiere: None,
            timer_id: None,
            current_program: None,
        }
    }
}
