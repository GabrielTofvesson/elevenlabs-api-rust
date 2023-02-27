use serde::{Serialize, Deserialize};

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
    history_item_id: String,
    voice_id: String,
    text: String,
    date_unix: u64,
    character_count_change_from: u32,
    character_count_change_to: u32,
    content_type: String,
    state: State,
    //settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    history: Vec<Item>,
}

#[derive(Debug, Serialize)]
pub struct HistoryItems {
    history_item_ids: Vec<String>,
}