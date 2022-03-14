pub mod action;
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

macro_rules! error_string {
    ($s:expr, $m:expr) => {
        concat!(
            r#"{"data":null,"success":false,"status_code":"#,
            $s,
            r#","error":""#,
            $m,
            "\"}"
        )
    };
}

/// status_code: `500`
#[allow(dead_code)]
pub const SERDE_ERROR: &str = error_string!(500, r#"JSON: Error converting to string"#);

/// status_code: `501`
#[allow(dead_code)]
pub const NOT_IMPLEMENTED_RELEASE_MODE: &str =
    error_string!(501, r#"Not implemented in release mode"#);

/// status_code: `500`
#[allow(dead_code)]
pub const INTERNAL_SERVER_ERROR: &str = error_string!(500, r#"INTERNAL_SERVER_ERROR"#);

/// status_code: `403`
#[allow(dead_code)]
pub const SPOTIFY_API_FORBIDDEN: &str =
    error_string!(403, r#"Non-premium accounts can't access Spotify API"#);

/// status_code: `400`
#[allow(dead_code)]
pub const SPOTIFY_API_SEARCH_MISSING_FIELDS: &str =
    error_string!(400, r#"There are missing fields"#);

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
