//! Artist operations
use crate::auth::AuthHeader;
use crate::http::HttpClient;
use crate::model::BaseItemDto;

use derive_builder::Builder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use super::{ClientResult, ItemResponse, Jellyfin};

impl Jellyfin {
    /// Retrieves artists or album artists
    pub async fn artists(
        &self,
        artist_query: &ArtistQuery,
        artist_type: &ArtistType,
    ) -> ClientResult<ItemResponse<BaseItemDto>> {
        let url: String;

        match artist_type {
            ArtistType::NonAlbumArtist => url = format!("{}/{}", self.base_url, "artists"),
            ArtistType::AlbumArtist => {
                url = format!("{}/{}/{}", self.base_url, "artists", "albumartists")
            }
        }

        let min_community_rating = artist_query.min_community_rating.map(|x| x.to_string());
        let start_index = artist_query.start_index.map(|x| x.to_string());
        let limit = artist_query.limit.map(|x| x.to_string());
        let fields = artist_query
            .fields
            .as_ref()
            .map(|x| x.into_iter().join(","));
        let exclude_item_types = artist_query
            .exclude_item_types
            .as_ref()
            .map(|x| x.join(","));
        let include_item_types = artist_query
            .include_item_types
            .as_ref()
            .map(|x| x.join(","));
        let filters = artist_query
            .filters
            .as_ref()
            .map(|x| x.into_iter().join(","));
        let is_favorite = artist_query.is_favorite.map(|x| x.to_string());
        let media_types = artist_query.media_types.as_ref().map(|x| x.join(","));
        let genres = artist_query.genres.as_ref().map(|x| x.join(","));
        let official_ratings = artist_query.official_ratings.as_ref().map(|x| x.join(","));
        let tags = artist_query.tags.as_ref().map(|x| x.join(","));
        let years = artist_query.years.as_ref().map(|x| x.into_iter().join(","));
        let enable_user_data = artist_query.enable_user_data.map(|x| x.to_string());
        let image_type_limit = artist_query.image_type_limit.map(|x| x.to_string());
        let enable_image_types = artist_query
            .enable_image_types
            .as_ref()
            .map(|x| x.into_iter().join(","));
        let person_ids = artist_query.person_ids.as_ref().map(|x| x.join(","));
        let person_types = artist_query.person_types.as_ref().map(|x| x.join(","));
        let studios = artist_query.studios.as_ref().map(|x| x.join(","));
        let studio_ids = artist_query.studio_ids.as_ref().map(|x| x.join(","));
        let enable_images = artist_query.enable_images.map(|x| x.to_string());
        let enable_total_record_count = artist_query
            .enable_total_record_count
            .map(|x| x.to_string());

        let params = crate::build_map! (
            optional "minCommunityRating": min_community_rating.as_deref(),
            optional "startIndex": start_index.as_deref(),
            optional "limit": limit.as_deref(),
            optional "searchTerm": artist_query.search_term.as_deref(),
            optional "parentId": artist_query.parent_id.as_deref(),
            optional "fields": fields.as_deref(),
            optional "excludeItemTypes": exclude_item_types.as_deref(),
            optional "includeItemTypes": include_item_types.as_deref(),
            optional "filters": filters.as_deref(),
            optional "isFavorite": is_favorite.as_deref(),
            optional "mediaTypes": media_types.as_deref(),
            optional "genres": genres.as_deref(),
            optional "officialRatings": official_ratings.as_deref(),
            optional "tags": tags.as_deref(),
            optional "years": years.as_deref(),
            optional "enableUserData": enable_user_data.as_deref(),
            optional "imageTypeLimit": image_type_limit.as_deref(),
            optional "enableImageTypes": enable_image_types.as_deref(),
            optional "person": artist_query.person.as_deref(),
            optional "personIds": person_ids.as_deref(),
            optional "personTypes": person_types.as_deref(),
            optional "studios": studios.as_deref(),
            optional "studio_ids": studio_ids.as_deref(),
            optional "user_id": artist_query.user_id.as_deref(),
            optional "nameStartsWithOrGreater": artist_query.name_starts_with_or_greater.as_deref(),
            optional "nameStartsWith": artist_query.name_starts_with.as_deref(),
            optional "nameLessThan": artist_query.name_less_than.as_deref(),
            optional "enableImages": enable_images.as_deref(),
            optional "enableTotalRecordCount": enable_total_record_count.as_deref(),
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
        let artists = serde_json::from_str(response.body())?;
        Ok(artists)
    }

    /// Retrieves a single artist by name
    pub async fn artist(&self, name: &str, user_id: Option<&str>) -> ClientResult<BaseItemDto> {
        let url = format!("{}/{}/{}", self.base_url, "artists", encode(name));
        let user_id = user_id.map(|x| x.to_string());
        let params = build_map!(
            optional "userId": user_id.as_deref(),
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
        let artist = serde_json::from_str(response.body())?;
        Ok(artist)
    }
}

/// Data representing a query for an artist
#[derive(Builder, Default, Debug, Clone)]
#[builder(setter(into, strip_option), default)]
pub struct ArtistQuery {
    min_community_rating: Option<f32>,
    start_index: Option<u32>,
    limit: Option<u32>,
    search_term: Option<String>,
    parent_id: Option<String>,
    fields: Option<Vec<ArtistQueryField>>,
    exclude_item_types: Option<Vec<String>>,
    include_item_types: Option<Vec<String>>,
    filters: Option<Vec<ArtistQueryFilter>>,
    is_favorite: Option<bool>,
    media_types: Option<Vec<String>>,
    genres: Option<Vec<String>>,
    official_ratings: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    years: Option<Vec<u32>>,
    enable_user_data: Option<bool>,
    image_type_limit: Option<u32>,
    enable_image_types: Option<Vec<ArtistQueryEnableImageType>>,
    person: Option<String>,
    person_ids: Option<Vec<String>>,
    person_types: Option<Vec<String>>,
    studios: Option<Vec<String>>,
    studio_ids: Option<Vec<String>>,
    user_id: Option<String>,
    name_starts_with_or_greater: Option<String>,
    name_starts_with: Option<String>,
    name_less_than: Option<String>,
    enable_images: Option<bool>,
    enable_total_record_count: Option<bool>,
}

/// A type of field that can be chosen in an artist query
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ArtistQueryField {
    ChildCount,
    CanDelete,
    CanDownload,
    ChannelInfo,
    Chapters,
    CumulativeRunTimeTicks,
    CustomRating,
    DateCreated,
    DateLastMediaAdded,
    DisplayPreferencesId,
    Etag,
    ExternalUrls,
    Genres,
    HomePageUrl,
    ItemCounts,
    MediaSourceCount,
    MediaSources,
    OriginalTitle,
    Overview,
    ParentId,
    Path,
    People,
    PlayAccess,
    ProductionLocations,
    ProviderIds,
    PrimaryImageAspectRatio,
    RecursiveItemCount,
    Settings,
    ScreenshotImageTags,
    SeriesPrimaryImage,
    SeriesStudio,
    SortName,
    SpecialEpisodeNumbers,
    Studios,
    BasicSyncInfo,
    SyncInfo,
    Taglines,
    Tags,
    RemoteTrailers,
    MediaStreams,
    SeasonUserData,
    ServiceName,
    ThemeSongIds,
    ThemeVideoIds,
    ExternalEtag,
    PresentationUniqueKey,
    InheritedParentalRatingValue,
    ExternalSeriesId,
    SeriesPresentationUniqueKey,
    DateLastRefreshed,
    DateLastSaved,
    RefreshState,
    ChannelImage,
    EnableMediaSourceDisplay,
    Width,
    Height,
    ExtraIds,
    LocalTrailerCount,
    IsHD,
    SpecialFeatureCount,
}

impl core::fmt::Display for ArtistQueryField {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A type of filter that can be chosen in an artist query
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArtistQueryFilter {
    IsFolder,
    IsNotFolder,
    IsUnplayed,
    IsPlayed,
    IsFavorite,
    IsResumable,
    Likes,
    Dislikes,
    IsFavoriteOrLikes,
}

impl core::fmt::Display for ArtistQueryFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A type of image that can be enabled in an artist query
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArtistQueryEnableImageType {
    Primary,
    Art,
    Backdrop,
    Banner,
    Logo,
    Thumb,
    Disc,
    Box,
    Screenshot,
    Menu,
    Chapter,
    BoxRear,
    Profile,
}

impl core::fmt::Display for ArtistQueryEnableImageType {
    fn fmt(&self, t: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(t, "{:?}", self)
    }
}

pub enum ArtistType {
    AlbumArtist,
    NonAlbumArtist,
}
