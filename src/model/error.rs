use serde::Deserialize;

use super::history::AudioExtractionError;

#[derive(Debug, Deserialize)]
pub enum Location {
    StringLocation(String),
    NumberLocation(u32),
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<Location>,
    pub msg: String,

    #[serde(rename = "type")]
    pub error_type: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTPValidationError {
    pub detail: Vec<ValidationError>,
}

#[derive(Debug)]
pub enum APIError {
    HTTPError(HTTPValidationError),
    NetworkError(reqwest::Error),
}

impl From<reqwest::Error> for APIError {
    fn from(error: reqwest::Error) -> Self {
        Self::NetworkError(error)
    }
}

impl From<HTTPValidationError> for APIError {
    fn from(error: HTTPValidationError) -> Self {
        Self::HTTPError(error)
    }
}

#[derive(Debug)]
pub enum ZippedAudioApiError {
    AudioExtractionError(AudioExtractionError),
    APIError(APIError),
}

impl From<AudioExtractionError> for ZippedAudioApiError {
    fn from(error: AudioExtractionError) -> Self {
        Self::AudioExtractionError(error)
    }
}

impl From<APIError> for ZippedAudioApiError {
    fn from(error: APIError) -> Self {
        Self::APIError(error)
    }
}