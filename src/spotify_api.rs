use crate::models::spotify_api;
use crate::models::spotify_api::{SpotifyMe, SpotifyTokens};
use crate::models::GenericOutput;
use actix_web::client::Client;
use std::env;

const REDIRECT_URL: &str = "http://127.0.0.1:8080/callback.html";
const ENDPOINT_AUTH_TOKEN: &str = "https://accounts.spotify.com/api/token";
const ENDPOINT_ME: &str = "https://api.spotify.com/v1/me";

type SpotifyResult<T> = Result<GenericOutput<T>, Box<dyn std::error::Error>>;

pub async fn api_spotify_authenticate(code: String) -> SpotifyResult<SpotifyTokens> {
    let client = Client::default();

    // Create request builder and send request
    let data = spotify_api::Authenticate {
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

    Ok(GenericOutput {
        data: Some(result),
        error: None,
        success: true,
        status_code: 200,
    })
}

pub async fn api_spotify_me(access_token: String) -> SpotifyResult<SpotifyMe> {
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

    Ok(GenericOutput {
        data: Some(result),
        error: None,
        success: true,
        status_code: 200,
    })
}
