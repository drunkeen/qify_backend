pub mod room;
pub mod song;
pub mod spotify_api;
pub mod spotify_id;

use serde::{Deserialize, Serialize};

pub type ServiceResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct GenericOutput<T: Serialize> {
    pub data: Option<T>,
    pub success: bool,
    pub status_code: u16,
    pub error: Option<String>,
}

#[allow(dead_code)]
pub const SERDE_ERROR: &str = r#"JSON: Error converting to string"#; // 500

#[allow(dead_code)]
pub const NOT_IMPLEMENTED_RELEASE_MODE: &str = r#"Not implemented in release mode"#; // 501

#[allow(dead_code)]
pub const INTERNAL_SERVER_ERROR: &str = r#"INTERNAL_SERVER_ERROR"#; // 500

#[allow(dead_code)]
pub const SPOTIFY_API_FORBIDDEN: &str = r#"Non-premium accounts can't access Spotify API"#; // 403

#[test]
fn generic_output_serialize() {
    let obj = GenericOutput {
        data: Some("Very good test"),
        success: true,
        error: None,
        status_code: 200,
    };

    let input = serde_json::to_string(&obj).unwrap();
    let expected = r#"{"data":"Very good test","success":true,"status_code":200,"error":null}"#;

    assert_eq!(input, expected);
}
