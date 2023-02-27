use bytes::Bytes;

use crate::{elevenlabs_api::{ElevenLabsAPI, self}, model::{history::{HistoryItems, HistoryResponse, AudioItem}, error::{APIError, HTTPValidationError, ZippedAudioApiError}}};

impl ElevenLabsAPI {
    pub async fn download_history_raw(&self, items: HistoryItems) -> Result<Bytes, APIError> {
        let response = self.post(elevenlabs_api::history::POST::DownloadHistory)?.json(&items).send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }

    pub async fn download_history(&self, items: HistoryItems) -> Result<AudioItem, ZippedAudioApiError> {
        let count = items.history_item_ids.len();
        let response = self.download_history_raw(items).await?;

        Ok(if count == 1 {
            AudioItem::new_single(response)
        } else {
            AudioItem::new_zipped(response)?
        })
    }

    pub async fn delete_history(&self, item: String) -> Result<String, APIError> {
        let response = self.delete(elevenlabs_api::history::DELETE::HistoryItem { item_id: item })?.send().await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }

    pub async fn get_history_audio(&self, item: String) -> Result<Bytes, APIError> {
        let response = self.get(elevenlabs_api::history::GET::HistoryItem { item_id: item })?.send().await?;

        if response.status().is_success() {
            Ok(response.bytes().await?)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }

    pub async fn get_history_items(&self) -> Result<HistoryResponse, APIError> {
        let response = self.get(elevenlabs_api::history::GET::History)?.send().await?;

        if response.status().is_success() {
            let history_items: HistoryResponse = response.json().await?;
            Ok(history_items)
        } else {
            let error: HTTPValidationError = response.json().await?;
            Err(APIError::HTTPError(error))
        }
    }
}