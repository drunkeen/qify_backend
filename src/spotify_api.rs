use actix_web::client::Client;
use std::env;

use crate::models::spotify_api::{Authenticate, SpotifyMe, SpotifySearch, SpotifyTokens};

const REDIRECT_URL: &str = "http://127.0.0.1:8080/callback.html";
const ENDPOINT_AUTH_TOKEN: &str = "https://accounts.spotify.com/api/token";
const ENDPOINT_ME: &str = "https://api.spotify.com/v1/me";
const ENDPOINT_SEARCH: &str = "https://api.spotify.com/v1/search";

type SpotifyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub async fn api_spotify_authenticate(code: String) -> SpotifyResult<SpotifyTokens> {
    let client = Client::default();

    // Create request builder and send request
    let data = Authenticate {
        grant_type: String::from("authorization_code"),
        code,
        redirect_uri: String::from(REDIRECT_URL),
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
    };

    let data = serde_urlencoded::ser::to_string(data).unwrap();

    let mut response = client
        .post(ENDPOINT_AUTH_TOKEN)
        .content_type("application/x-www-form-urlencoded")
        .send_body(&data) // <- Send request
        .await?; // <- Wait for response

    let result = response.body().await?;
    let result: &str = std::str::from_utf8(&*result)?;
    let result = serde_json::from_str::<SpotifyTokens>(result)?;

    Ok(result)
}

pub async fn api_spotify_me(access_token: &str) -> SpotifyResult<SpotifyMe> {
    let client = Client::default();

    // Create request builder and send request
    let mut response = client
        .get(ENDPOINT_ME)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?; // <- Wait for response

    let result = response.body().await?;
    let result: &str = std::str::from_utf8(&*result)?;
    let result = serde_json::from_str::<SpotifyMe>(result)?;

    Ok(result)
}

pub async fn api_spotify_search(
    access_token: &str,
    query: &str,
    offset: u16,
) -> SpotifyResult<SpotifySearch> {
    let client = Client::default();

    // Create request builder and send request
    let mut response = client
        .get(format!(
            "{ENDPOINT_SEARCH}?q={query}&type={type_}&market={market}&limit={limit}&offset={offset}",
            ENDPOINT_SEARCH = ENDPOINT_SEARCH,
            query = query,
            type_ = "track",
            market = "FR",
            limit = 10,
            offset = offset
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?; // <- Wait for response

    let result = response.body().await?;
    let result: &str = std::str::from_utf8(&*result)?;

    let result = serde_json::from_str::<SpotifySearch>(result)?;

    Ok(result)
}
