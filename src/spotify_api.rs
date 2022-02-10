use crate::models::GenericOutput;
use actix_web::client::Client;
use awc::error::SendRequestError;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::ErrorKind;

const REDIRECT_URL: &str = "http://127.0.0.1:8080/callback.html";
const ENDPOINT_AUTH_TOKEN: &str = "https://accounts.spotify.com/api/token";

#[derive(Serialize, Deserialize)]
struct Authenticate {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpotifyTokens {
    access_token: String,
    token_type: String,
    expires_in: u16,
    refresh_token: String,
    scope: String,
}

pub struct SpotifyId {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u16,
    refresh_token: String,
}

pub async fn api_spotify_authenticate(
    code: String,
) -> Result<GenericOutput<SpotifyTokens>, Box<dyn std::error::Error>> {
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

    let data = response.body().await?;
    let data: &str = std::str::from_utf8(&*data)?;
    let data = serde_json::from_str::<SpotifyTokens>(data)?;

    Ok(GenericOutput {
        data: Some(data),
        error: None,
        success: true,
        status_code: 200,
    })
}
