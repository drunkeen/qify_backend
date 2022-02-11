use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Authenticate {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyTokens {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyFollowers {
    pub href: Option<String>,
    pub total: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyImage {
    pub url: String,
    pub height: u16,
    pub width: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyMeError {
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    pub code: String,
}
