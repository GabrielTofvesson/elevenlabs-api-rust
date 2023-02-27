use std::io::Read;

use bytes::Bytes;
use serde::{Serialize, Deserialize};
use zip::result::ZipError;

#[derive(Debug, Deserialize)]
pub enum State {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "processing")]
    Processing,
}

#[derive(Debug, Deserialize)]
pub struct Settings;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub history_item_id: String,
    pub voice_id: String,
    pub text: String,
    pub date_unix: u64,
    pub character_count_change_from: u32,
    pub character_count_change_to: u32,
    pub content_type: String,
    pub state: State,
    //pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    pub history: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub struct HistoryItems {
    pub history_item_ids: Vec<String>,
}

#[derive(Debug)]
pub enum AudioExtractionError {
    IoError(std::io::Error),
    ZipError(ZipError),
}

impl From<ZipError> for AudioExtractionError {
    fn from(error: ZipError) -> Self {
        Self::ZipError(error)
    }
}

impl From<std::io::Error> for AudioExtractionError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

#[derive(Debug)]
pub struct AudioItem {
    data: Vec<Vec<u8>>,
}

impl AudioItem {
    pub(crate) fn new_single(audio: Bytes) -> Self {
        Self {
            data: vec![audio.to_vec()],
        }
    }

    pub(crate) fn new_zipped(audio: Bytes) -> Result<Self, AudioExtractionError> {
        let reader = std::io::Cursor::new(&audio);

        let mut zip = zip::ZipArchive::new(reader)?;
        let mut collect = Vec::new();
        for index in 0..zip.len() {
            let mut buffer = Vec::new();
            let mut file = zip.by_index(index)?;
            file.read_to_end(&mut buffer)?; // TODO: Unwrap?
            collect.push(buffer);
        }

        Ok(Self {
            data: collect,
        })
    }

    pub fn audio(&self) -> &Vec<Vec<u8>> {
        &self.data
    }
}