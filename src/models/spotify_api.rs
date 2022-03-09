use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Authenticate {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyFollowers {
    pub href: Option<String>,
    pub total: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyImage {
    pub url: String,
    pub height: u16,
    pub width: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyMeError {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyMe {
    pub country: String,
    pub display_name: String,
    pub email: String,
    pub explicit_content: SpotifyExplicitContent,
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub product: String,
    pub r#type: String,
    pub uri: String,
    pub error: Option<SpotifyMeError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyExternalId {
    pub isrc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyArtist {
    pub external_urls: SpotifyExternalUrls,
    pub followers: SpotifyFollowers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,

    pub images: Vec<SpotifyImage>,

    pub name: String,
    pub popularity: u16,
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTrackArtist {
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTrackAlbum {
    pub album_type: String,
    pub artists: Vec<SpotifyTrackArtist>, // TODO: Update type
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Vec<SpotifyImage>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u16,
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTrack {
    pub album: SpotifyTrackAlbum,
    pub artists: Vec<SpotifyTrackArtist>,
    pub disc_number: u8,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: SpotifyExternalId,
    pub external_urls: SpotifyExternalUrls,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: u16,
    pub preview_url: Option<String>,
    pub track_number: u16,
    pub r#type: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyTrackFiltered {
    pub title: String,         // name
    pub image: Option<String>, // album.images.first()
    pub album: String,         // album.name
    pub uri: String,
    pub duration_ms: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifySearchResult<T> {
    pub limit: u16,
    pub offset: u16,
    pub total: u16,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifySearch {
    // pub artist: SpotifySearchResult<SpotifyArtist>,
    pub tracks: SpotifySearchResult<SpotifyTrack>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshAccountData {
    pub grant_type: &'static str,
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshAccountResult {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}
