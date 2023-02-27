use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Location {
    StringLocation(String),
    NumberLocation(u32),
}

#[derive(Debug, Deserialize)]
pub struct ValidationError {
    loc: Vec<Location>,
    msg: String,

    #[serde(rename = "type")]
    error_type: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTPValidationError {
    detail: Vec<ValidationError>,
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