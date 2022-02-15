pub mod room;
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
fn serde_error_deserialize() {
    let serde = serde_json::from_str::<GenericOutput<u8>>(&SERDE_ERROR);
    if let Ok(result) = serde {
        let expected: GenericOutput<u8> = GenericOutput {
            data: None,
            status_code: 500,
            success: false,
            error: Some(String::from("JSON: Error converting to string")),
        };

        assert!(result == expected);
    } else if let Err(result) = serde {
        panic!("\n{}\n", result);
    }
}

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