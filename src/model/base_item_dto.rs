use serde::{Deserialize, Serialize};
/// BaseItemDto : This is strictly used as a data transfer object from the api layer.  This holds information about a BaseItem in a format that is convenient for the client

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseItemDto {
    /// Gets or sets the name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginalTitle", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    /// Gets or sets the server identifier.
    #[serde(rename = "ServerId", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// Gets or sets the id.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Gets or sets the etag.
    #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// Gets or sets the type of the source.
    #[serde(rename = "SourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// Gets or sets the playlist item identifier.
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option::is_none")]
    pub playlist_item_id: Option<String>,
    /// Gets or sets the date created.
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
    /// Gets or sets a value indicating whether [supports synchronize].
    #[serde(rename = "SupportsSync", skip_serializing_if = "Option::is_none")]
    pub supports_sync: Option<bool>,
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Gets or sets the name of the sort.
    #[serde(rename = "SortName", skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    #[serde(rename = "ForcedSortName", skip_serializing_if = "Option::is_none")]
    pub forced_sort_name: Option<String>,
    /// Gets or sets the video3 D format.
    #[serde(rename = "Video3DFormat", skip_serializing_if = "Option::is_none")]
    pub video3_d_format: Option<Box<crate::model::Video3DFormat>>,
    /// Gets or sets the premiere date.
    #[serde(rename = "PremiereDate", skip_serializing_if = "Option::is_none")]
    pub premiere_date: Option<String>,
    /// Gets or sets the external urls.
    #[serde(rename = "ExternalUrls", skip_serializing_if = "Option::is_none")]
    pub external_urls: Option<Vec<crate::model::ExternalUrl>>,
    /// Gets or sets the media versions.
    #[serde(rename = "MediaSources", skip_serializing_if = "Option::is_none")]
    pub media_sources: Option<Vec<crate::model::MediaSourceInfo>>,
    /// Gets or sets the critic rating.
    #[serde(rename = "CriticRating", skip_serializing_if = "Option::is_none")]
    pub critic_rating: Option<f32>,
    #[serde(
        rename = "ProductionLocations",
        skip_serializing_if = "Option::is_none"
    )]
    pub production_locations: Option<Vec<String>>,
    /// Gets or sets the path.
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "EnableMediaSourceDisplay",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_media_source_display: Option<bool>,
    /// Gets or sets the official rating.
    #[serde(rename = "OfficialRating", skip_serializing_if = "Option::is_none")]
    pub official_rating: Option<String>,
    /// Gets or sets the custom rating.
    #[serde(rename = "CustomRating", skip_serializing_if = "Option::is_none")]
    pub custom_rating: Option<String>,
    /// Gets or sets the channel identifier.
    #[serde(rename = "ChannelId", skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// Gets or sets the overview.
    #[serde(rename = "Overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    /// Gets or sets the taglines.
    #[serde(rename = "Taglines", skip_serializing_if = "Option::is_none")]
    pub taglines: Option<Vec<String>>,
    /// Gets or sets the genres.
    #[serde(rename = "Genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<String>>,
    /// Gets or sets the community rating.
    #[serde(rename = "CommunityRating", skip_serializing_if = "Option::is_none")]
    pub community_rating: Option<f32>,
    /// Gets or sets the cumulative run time ticks.
    #[serde(
        rename = "CumulativeRunTimeTicks",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_run_time_ticks: Option<i64>,
    /// Gets or sets the run time ticks.
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    /// Gets or sets the play access.
    #[serde(rename = "PlayAccess", skip_serializing_if = "Option::is_none")]
    pub play_access: Option<Box<crate::model::PlayAccess>>,
    /// Gets or sets the aspect ratio.
    #[serde(rename = "AspectRatio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    /// Gets or sets the production year.
    #[serde(rename = "ProductionYear", skip_serializing_if = "Option::is_none")]
    pub production_year: Option<i32>,
    /// Gets or sets a value indicating whether this instance is place holder.
    #[serde(rename = "IsPlaceHolder", skip_serializing_if = "Option::is_none")]
    pub is_place_holder: Option<bool>,
    /// Gets or sets the number.
    #[serde(rename = "Number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "ChannelNumber", skip_serializing_if = "Option::is_none")]
    pub channel_number: Option<String>,
    /// Gets or sets the index number.
    #[serde(rename = "IndexNumber", skip_serializing_if = "Option::is_none")]
    pub index_number: Option<i32>,
    /// Gets or sets the index number end.
    #[serde(rename = "IndexNumberEnd", skip_serializing_if = "Option::is_none")]
    pub index_number_end: Option<i32>,
    /// Gets or sets the parent index number.
    #[serde(rename = "ParentIndexNumber", skip_serializing_if = "Option::is_none")]
    pub parent_index_number: Option<i32>,
    /// Gets or sets the trailer urls.
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option::is_none")]
    pub remote_trailers: Option<Vec<crate::model::MediaUrl>>,
    /// Gets or sets the provider ids.
    #[serde(rename = "ProviderIds", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<::std::collections::HashMap<String, String>>,
    /// Gets or sets a value indicating whether this instance is HD.
    #[serde(rename = "IsHD", skip_serializing_if = "Option::is_none")]
    pub is_hd: Option<bool>,
    /// Gets or sets a value indicating whether this instance is folder.
    #[serde(rename = "IsFolder", skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    /// Gets or sets the parent id.
    #[serde(rename = "ParentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// Gets or sets the type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Gets or sets the people.
    #[serde(rename = "People", skip_serializing_if = "Option::is_none")]
    pub people: Option<Vec<crate::model::BaseItemPerson>>,
    /// Gets or sets the studios.
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
    /// Gets or sets the parent backdrop image tags.
    #[serde(
        rename = "ParentBackdropImageTags",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_backdrop_image_tags: Option<Vec<String>>,
    /// Gets or sets the local trailer count.
    #[serde(rename = "LocalTrailerCount", skip_serializing_if = "Option::is_none")]
    pub local_trailer_count: Option<i32>,
    /// User data for this item based on the user it's being requested for.
    #[serde(rename = "UserData", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Box<crate::model::UserItemDataDto>>,
    /// Gets or sets the recursive item count.
    #[serde(rename = "RecursiveItemCount", skip_serializing_if = "Option::is_none")]
    pub recursive_item_count: Option<i32>,
    /// Gets or sets the child count.
    #[serde(rename = "ChildCount", skip_serializing_if = "Option::is_none")]
    pub child_count: Option<i32>,
    /// Gets or sets the name of the series.
    #[serde(rename = "SeriesName", skip_serializing_if = "Option::is_none")]
    pub series_name: Option<String>,
    /// Gets or sets the series id.
    #[serde(rename = "SeriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<String>,
    /// Gets or sets the season identifier.
    #[serde(rename = "SeasonId", skip_serializing_if = "Option::is_none")]
    pub season_id: Option<String>,
    /// Gets or sets the special feature count.
    #[serde(
        rename = "SpecialFeatureCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub special_feature_count: Option<i32>,
    /// Gets or sets the display preferences id.
    #[serde(
        rename = "DisplayPreferencesId",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_preferences_id: Option<String>,
    /// Gets or sets the status.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Gets or sets the air time.
    #[serde(rename = "AirTime", skip_serializing_if = "Option::is_none")]
    pub air_time: Option<String>,
    /// Gets or sets the air days.
    #[serde(rename = "AirDays", skip_serializing_if = "Option::is_none")]
    pub air_days: Option<Vec<crate::model::DayOfWeek>>,
    /// Gets or sets the tags.
    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Gets or sets the primary image aspect ratio, after image enhancements.
    #[serde(
        rename = "PrimaryImageAspectRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
    /// Gets or sets the artists.
    #[serde(rename = "Artists", skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
    /// Gets or sets the artist items.
    #[serde(rename = "ArtistItems", skip_serializing_if = "Option::is_none")]
    pub artist_items: Option<Vec<crate::model::NameGuidPair>>,
    /// Gets or sets the album.
    #[serde(rename = "Album", skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    /// Gets or sets the type of the collection.
    #[serde(rename = "CollectionType", skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<String>,
    /// Gets or sets the display order.
    #[serde(rename = "DisplayOrder", skip_serializing_if = "Option::is_none")]
    pub display_order: Option<String>,
    /// Gets or sets the album id.
    #[serde(rename = "AlbumId", skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    /// Gets or sets the album image tag.
    #[serde(
        rename = "AlbumPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub album_primary_image_tag: Option<String>,
    /// Gets or sets the series primary image tag.
    #[serde(
        rename = "SeriesPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub series_primary_image_tag: Option<String>,
    /// Gets or sets the album artist.
    #[serde(rename = "AlbumArtist", skip_serializing_if = "Option::is_none")]
    pub album_artist: Option<String>,
    /// Gets or sets the album artists.
    #[serde(rename = "AlbumArtists", skip_serializing_if = "Option::is_none")]
    pub album_artists: Option<Vec<crate::model::NameGuidPair>>,
    /// Gets or sets the name of the season.
    #[serde(rename = "SeasonName", skip_serializing_if = "Option::is_none")]
    pub season_name: Option<String>,
    /// Gets or sets the media streams.
    #[serde(rename = "MediaStreams", skip_serializing_if = "Option::is_none")]
    pub media_streams: Option<Vec<crate::model::MediaStream>>,
    /// Gets or sets the type of the video.
    #[serde(rename = "VideoType", skip_serializing_if = "Option::is_none")]
    pub video_type: Option<Box<crate::model::VideoType>>,
    /// Gets or sets the part count.
    #[serde(rename = "PartCount", skip_serializing_if = "Option::is_none")]
    pub part_count: Option<i32>,
    #[serde(rename = "MediaSourceCount", skip_serializing_if = "Option::is_none")]
    pub media_source_count: Option<i32>,
    /// Gets or sets the image tags.
    #[serde(rename = "ImageTags", skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<::std::collections::HashMap<String, String>>,
    /// Gets or sets the backdrop image tags.
    #[serde(rename = "BackdropImageTags", skip_serializing_if = "Option::is_none")]
    pub backdrop_image_tags: Option<Vec<String>>,
    /// Gets or sets the screenshot image tags.
    #[serde(
        rename = "ScreenshotImageTags",
        skip_serializing_if = "Option::is_none"
    )]
    pub screenshot_image_tags: Option<Vec<String>>,
    /// Gets or sets the parent logo image tag.
    #[serde(rename = "ParentLogoImageTag", skip_serializing_if = "Option::is_none")]
    pub parent_logo_image_tag: Option<String>,
    /// If the item does not have a art, this will hold the Id of the Parent that has one.
    #[serde(rename = "ParentArtItemId", skip_serializing_if = "Option::is_none")]
    pub parent_art_item_id: Option<String>,
    /// Gets or sets the parent art image tag.
    #[serde(rename = "ParentArtImageTag", skip_serializing_if = "Option::is_none")]
    pub parent_art_image_tag: Option<String>,
    /// Gets or sets the series thumb image tag.
    #[serde(
        rename = "SeriesThumbImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub series_thumb_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option::is_none")]
    pub image_blur_hashes: Option<Box<crate::model::BaseItemDtoImageBlurHashes>>,
    /// Gets or sets the series studio.
    #[serde(rename = "SeriesStudio", skip_serializing_if = "Option::is_none")]
    pub series_studio: Option<String>,
    /// Gets or sets the parent thumb item id.
    #[serde(rename = "ParentThumbItemId", skip_serializing_if = "Option::is_none")]
    pub parent_thumb_item_id: Option<String>,
    /// Gets or sets the parent thumb image tag.
    #[serde(
        rename = "ParentThumbImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_thumb_image_tag: Option<String>,
    /// Gets or sets the parent primary image item identifier.
    #[serde(
        rename = "ParentPrimaryImageItemId",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_item_id: Option<String>,
    /// Gets or sets the parent primary image tag.
    #[serde(
        rename = "ParentPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_primary_image_tag: Option<String>,
    /// Gets or sets the chapters.
    #[serde(rename = "Chapters", skip_serializing_if = "Option::is_none")]
    pub chapters: Option<Vec<crate::model::ChapterInfo>>,
    /// Gets or sets the type of the location.
    #[serde(rename = "LocationType", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<Box<crate::model::LocationType>>,
    /// Gets or sets the type of the iso.
    #[serde(rename = "IsoType", skip_serializing_if = "Option::is_none")]
    pub iso_type: Option<Box<crate::model::IsoType>>,
    /// Gets or sets the type of the media.
    #[serde(rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Gets or sets the end date.
    #[serde(rename = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Gets or sets the locked fields.
    #[serde(rename = "LockedFields", skip_serializing_if = "Option::is_none")]
    pub locked_fields: Option<Vec<crate::model::MetadataField>>,
    /// Gets or sets the trailer count.
    #[serde(rename = "TrailerCount", skip_serializing_if = "Option::is_none")]
    pub trailer_count: Option<i32>,
    /// Gets or sets the movie count.
    #[serde(rename = "MovieCount", skip_serializing_if = "Option::is_none")]
    pub movie_count: Option<i32>,
    /// Gets or sets the series count.
    #[serde(rename = "SeriesCount", skip_serializing_if = "Option::is_none")]
    pub series_count: Option<i32>,
    #[serde(rename = "ProgramCount", skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i32>,
    /// Gets or sets the episode count.
    #[serde(rename = "EpisodeCount", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    /// Gets or sets the song count.
    #[serde(rename = "SongCount", skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    /// Gets or sets the album count.
    #[serde(rename = "AlbumCount", skip_serializing_if = "Option::is_none")]
    pub album_count: Option<i32>,
    #[serde(rename = "ArtistCount", skip_serializing_if = "Option::is_none")]
    pub artist_count: Option<i32>,
    /// Gets or sets the music video count.
    #[serde(rename = "MusicVideoCount", skip_serializing_if = "Option::is_none")]
    pub music_video_count: Option<i32>,
    /// Gets or sets a value indicating whether [enable internet providers].
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
    /// Gets or sets the series timer identifier.
    #[serde(rename = "SeriesTimerId", skip_serializing_if = "Option::is_none")]
    pub series_timer_id: Option<String>,
    /// Gets or sets the program identifier.
    #[serde(rename = "ProgramId", skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    /// Gets or sets the channel primary image tag.
    #[serde(
        rename = "ChannelPrimaryImageTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    /// The start date of the recording, in UTC.
    #[serde(rename = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Gets or sets the completion percentage.
    #[serde(
        rename = "CompletionPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_percentage: Option<f64>,
    /// Gets or sets a value indicating whether this instance is repeat.
    #[serde(rename = "IsRepeat", skip_serializing_if = "Option::is_none")]
    pub is_repeat: Option<bool>,
    /// Gets or sets the episode title.
    #[serde(rename = "EpisodeTitle", skip_serializing_if = "Option::is_none")]
    pub episode_title: Option<String>,
    /// Gets or sets the type of the channel.
    #[serde(rename = "ChannelType", skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<Box<crate::model::ChannelType>>,
    /// Gets or sets the audio.
    #[serde(rename = "Audio", skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<crate::model::ProgramAudio>>,
    /// Gets or sets a value indicating whether this instance is movie.
    #[serde(rename = "IsMovie", skip_serializing_if = "Option::is_none")]
    pub is_movie: Option<bool>,
    /// Gets or sets a value indicating whether this instance is sports.
    #[serde(rename = "IsSports", skip_serializing_if = "Option::is_none")]
    pub is_sports: Option<bool>,
    /// Gets or sets a value indicating whether this instance is series.
    #[serde(rename = "IsSeries", skip_serializing_if = "Option::is_none")]
    pub is_series: Option<bool>,
    /// Gets or sets a value indicating whether this instance is live.
    #[serde(rename = "IsLive", skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    /// Gets or sets a value indicating whether this instance is news.
    #[serde(rename = "IsNews", skip_serializing_if = "Option::is_none")]
    pub is_news: Option<bool>,
    /// Gets or sets a value indicating whether this instance is kids.
    #[serde(rename = "IsKids", skip_serializing_if = "Option::is_none")]
    pub is_kids: Option<bool>,
    /// Gets or sets a value indicating whether this instance is premiere.
    #[serde(rename = "IsPremiere", skip_serializing_if = "Option::is_none")]
    pub is_premiere: Option<bool>,
    /// Gets or sets the timer identifier.
    #[serde(rename = "TimerId", skip_serializing_if = "Option::is_none")]
    pub timer_id: Option<String>,
    /// Gets or sets the current program.
    #[serde(rename = "CurrentProgram", skip_serializing_if = "Option::is_none")]
    pub current_program: Option<Box<crate::model::BaseItemDto>>,
}

impl BaseItemDto {
    /// This is strictly used as a data transfer object from the api layer.  This holds information about a BaseItem in a format that is convenient for the client.
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
